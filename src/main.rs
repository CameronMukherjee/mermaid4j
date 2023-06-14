use std::{env, fs};
use std::collections::HashMap;
use std::fmt::format;
use std::io::{self, BufRead, BufReader, Write};

use regex::Regex;
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let current_dir = env::current_dir()?;
    let mut path = current_dir.clone();
    let mut directories = Vec::new();
    let mut use_partitioned_files = false;

    for (index, arg) in args.iter().enumerate() {
        if arg == "--partitioned" {
            use_partitioned_files = true;
        } else if index == 1 {
            path = std::path::PathBuf::from(arg);
        } else if index > 1 {
            directories.push(arg.clone());
        }
    }

    if directories.is_empty() {
        for entry in current_dir.read_dir()? {
            if let Ok(entry) = entry {
                if entry.file_type()?.is_dir() {
                    directories.push(entry.file_name().into_string().unwrap());
                }
            }
        }
    }

    let pattern = r"private final (\w+) \w+;";
    let regex = Regex::new(pattern).unwrap();

    let mut map: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();

    for directory in &directories {
        let current_directory = directory.to_string();
        map.insert(current_directory.clone(), HashMap::new());

        // Recursively iterate over all entries in the directory
        for entry in WalkDir::new(path.join(directory)) {
            let entry = entry?;

            // Skip files that are not readable or don't have the ".java" extension
            if !entry.file_type().is_file()
                || !entry.metadata()?.is_file()
                || entry.path().extension().map_or(true, |ext| ext != "java")
            {
                continue;
            }

            if entry.path().components().any(|c| c.as_os_str() == "build") {
                // Skip over "build" directories
                continue;
            }

            // Extract the filename from the file path
            let file_name = entry.file_name().to_string_lossy().to_string();

            // Open the file
            let file = fs::File::open(entry.path())?;
            let reader = BufReader::new(file);

            // Process the file contents line by line
            for line in reader.lines() {
                let line = line?;

                // Check if the line matches the regex pattern
                if let Some(captures) = regex.captures(&line) {
                    if let Some(class_name) = captures.get(1) {
                        if class_name.as_str() == "String" {
                            continue;
                        }
                        let inner_map = map.get_mut(&current_directory).unwrap();
                        let class_names = inner_map.entry(file_name.clone()).or_insert(Vec::new());
                        class_names.push(class_name.as_str().to_string());
                    }
                }
            }
        }
    }

    // Write the HashMap contents to .md files
    if use_partitioned_files {
        for (current_directory, inner_map) in map.iter() {
            for (file_name, class_names) in inner_map.iter() {
                let mut content = String::from("```mermaid\nclassDiagram\n");
                for class_name in class_names {
                    content += &format!("      {} <|-- {}\n", file_name.strip_suffix(".java").unwrap_or(file_name), class_name);
                }
                content += "```\n";
                fs::write(format!("diagrams/{}_{}.md", current_directory, file_name.strip_suffix(".java").unwrap_or(file_name)), content)?;
            }
        }
    } else {
        for (current_directory, inner_map) in map.iter() {
            let mut content = String::from("```mermaid\nclassDiagram\n");
            for (file_name, class_names) in inner_map.iter() {
                for class_name in class_names {
                    content += &format!("      {} <|-- {}\n", file_name.strip_suffix(".java").unwrap_or(file_name), class_name);
                }
            }
            content += "```\n";
            fs::write(format!("diagrams/{}.md", current_directory), content)?;
        }
    }

    Ok(())
}

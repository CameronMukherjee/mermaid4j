# Mermaid4J

This is a utility tool written in Rust that recursively searches through directories for Java files and generates class diagrams. These diagrams are created
using the Mermaid syntax and saved as markdown files. The tool identifies class attributes within each file, excluding Strings, and includes them in the class
diagram.

## Installation

To install this tool:

Go to the Releases section of this repository.
Download the latest release binary suitable for your operating system.
Extract the downloaded file and make the binary executable.
For Unix-like operating systems, you can use the following command:

```bash
Copy code
chmod +x mermaid4j 
```

Move the binary to a directory included in your PATH for easy access.

```bash
Copy code
mv mermaid4j /usr/local/bin/
```

## Usage

You can use this tool directly from the command line by running the binary followed by arguments representing the directories you want to scan. If no
directories are provided, the current directory will be scanned.

```bash
Copy code
mermaid4j <directory_1> <directory_2> ... <directory_n>
```

Alternatively, you can specify a path to scan using the --path flag:

```bash
Copy code
mermaid4j --path /path/to/scan
```

By default, the tool generates a single markdown file containing all diagrams for each scanned directory. If you want to generate a separate file for each Java
file, use the --partitioned flag:

```bash
mermaid4j --partitioned
```

The generated markdown files will be saved in a diagrams directory within the current working directory.

## Dependencies

This tool uses the following crates:

- std for general utility functions and I/O.
- regex for pattern matching within files.
- walkdir for recursive directory traversal.

## Contribution

Feel free to open an issue or submit a pull request if you find a bug or have a feature request. Contributions are always welcome!

## License

This project is licensed under the MIT license. See the LICENSE file for details.
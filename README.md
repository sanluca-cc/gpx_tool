# GPX Tool

This is a simple tool to do different operations with GPX files. It is written in Rust and uses the [GeoRust](https://georust.org/) ecosystem.

This project is a Cargo workspace with the following crates:

- `gpx_tool`: The main binary crate.
- `gpx_utils`: A library crate with utilities to read and write GPX files via a common data structure.
- `common`: A library with data structures and utilities shared between the other crates.
- `route_fixer`: A library crate with utilities to fix different parts of a route.

# Usage

The tool is a command line application. You can run it with the `--help` flag to see the available tools options.

When running the tool, you need to pass a GPX file as an argument. The tool will read the file, do the requested operation, and write the result to a new file. The name of the new file can be passed with the `--out` or `-o` flag. If no name is passed, the tool will write the result to a file with the same name as the input file, but with the suffix `-new` before the extension.

As of now, the only available tool is `route_fixer`. This tool can be used to fix different parts of a route. To use it, pass either the `--fix` or `-f` flag to the command.

Passing the `--debug` or `-d` flag will print debug information to the console.

# License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

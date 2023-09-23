# HtmxRustPriceWidget

## Overview

**HtmxRustPriceWidget** is a specialized widget designed to be integrated into larger financial projects. It serves as a self-contained module for generating buy/sell tables based on various algorithms. Written in Rust, it combines the power of multiple libraries to provide a comprehensive solution for both backend logic and frontend display.

## Features

- Generate buy/sell tables with customized parameters.
- Outputs data in multiple formats: Database, JSON, and PrettyTable.
- Built-in web server for interacting with the widget.
- Actix-based backend serving HTML templates.
  
## Dependencies

The project relies on several Rust crates, as specified in `Cargo.toml`. Some of the main ones include:

- `actix-web` for the web framework.
- `serde` for serialization and deserialization.
- `mysql` for database interactions.
- `prettytable` for displaying tables in the console.
- `tera` for template rendering.

## How to Run

1. **Compile the binaries**: Run `./build.sh` to compile the project.
2. **Start the Web Server**: Execute the compiled web server binary to start the Actix web server.  

        cd frontend && ./webserver

3. **Access the Widget**: Open a web browser and navigate to `http://127.0.0.1:5242`.

## Contribution and Integration

This widget is designed to be a modular component that can be integrated into larger projects. The focus on functional programming and the clean separation of concerns make it straightforward to integrate and extend.

For contributions and issues, please open a pull request or issue on the project repository.


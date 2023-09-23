#!/bin/bash

# Function to compile the project
compile_project() {
  cargo build
}

# Function to move compiled binaries
move_binaries() {
  mv ./target/debug/al_go2 ./frontend
  mv ./target/debug/webserver ./frontend
}

# Execute the functions
compile_project
move_binaries

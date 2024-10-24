Promptput

promptput is a simple and intuitive crate for handling terminal user input in Rust. It allows you to prompt the user with a message and capture their input as the desired data type. It is inspired by the `input()` method in Python.

Usage:
* Add to Cargo.toml -> `promptput = "0.1.0"`
* Import it -> `use promptput::input;`
* `let name: &str = input("Enter name: ");`
* `let age: i32 = input("Enter age: ");`
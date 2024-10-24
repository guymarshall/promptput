**Promptput**

Promptput is a simple and intuitive crate for handling terminal user input in Rust. It allows you to prompt the user with a message and capture their input as the desired data type. It is inspired by the `input()` method in Python.

**Installation**

Add this to your Cargo.toml:

```
[dependencies]
promptput = "0.1"
```

**Usage**

Here's a simple example of how to use promptput:

```
use promptput::input;

fn main() {
    let name: &str = input("Name: ");
    let age: i32 = input("Age: ");

    println!("Name: {name}, Age: {age}");
}
```

**How it Works**

The input() function takes a prompt &str and returns the user input, automatically converting it into the specified data type.

**Contributing**

Feel free to open issues or submit pull requests if you have any improvements or suggestions.
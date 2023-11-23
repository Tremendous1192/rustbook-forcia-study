// Import the custom procedural macro `tomlstruct` from the `tomlstruct` module
use tomlstruct::tomlstruct;

// Apply the procedural macro to define a struct named `Hello` with fields `name` and `version`
tomlstruct! {
    [Hello]
    name = "hello"
    version = 1.0
}

// Main function
fn main() {
    // Create an instance of the `Hello` struct
    let hello_proc = Hello {
        name: String::from("hello"),
        version: 1.0,
    };

    // Print the values of the struct fields
    println!("{}", &hello_proc.name);
    println!("{}", &hello_proc.version);
}

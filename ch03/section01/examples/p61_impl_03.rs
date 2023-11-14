// Define a struct named `Person` with fields `name` of type `String` and `age` of type `u32`
struct Person {
    name: String,
    age: u32,
}

// Implementation block for the `Person` struct
impl Person {
    // Define an associated function `new` for the `Person` struct, used to create a new instance
    // Takes a `name` parameter of type `&str` and an `age` parameter of type `u32`
    // Returns a new `Person` instance with the specified `name` and `age`
    fn new(name: &str, age: u32) -> Person {
        Person {
            name: String::from(name),
            age: age,
        }
    }

    // Define a method `say_name` for the `Person` struct, which prints the person's name
    // Returns a reference to `Self` to allow method chaining
    fn say_name(&self) -> &Self {
        println!("I am {}.", self.name);
        // Return a reference to `Self` to allow method chaining
        self
    }

    // Define a method `say_age` for the `Person` struct, which prints the person's age
    // Returns a reference to `Self` to allow method chaining
    fn say_age(&self) -> &Self {
        println!("I am {} year(s) old.", self.age);
        // Return a reference to `Self` to allow method chaining
        self
    }
}

// Main function where the program execution begins
fn main() {
    // Create an instance `p` of the `Person` struct using the associated function `new`
    // with a name "Taro" and age 20
    let p = Person::new("Taro", 20);

    // Call the `say_name` method on the `p` instance, print the person's name, and allow method chaining
    // Call the `say_age` method on the result, print the person's age
    p.say_name().say_age();
}

// Define a struct named `Person` with fields `name` of type `String` and `age` of type `u32`
struct Person {
    name: String,
    age: u32,
}

// Implementation block for the `Person` struct
impl Person {
    // Define a method `say_name` for the `Person` struct, which prints the person's name
    fn say_name(&self) {
        println!("I am {}.", self.name);
    }

    // Define a method `say_age` for the `Person` struct, which prints the person's age
    fn say_age(&self) {
        println!("I am {} year(s) old.", self.age);
    }
}

// Main function where the program execution begins
fn main() {
    // Create an instance `p` of the `Person` struct with a name "Taro" and age 20
    let p = Person {
        name: String::from("Taro"),
        age: 20,
    };

    // Call the `say_name` method on the `p` instance, printing the person's name
    p.say_name();

    // Call the `say_age` method on the `p` instance, printing the person's age
    p.say_age();
}

// Definition of an enumeration representing different emotions
enum Emotion {
    Anger,
    Happy,
}

// Definition of a trait named 'Emotional' to represent emotional states
trait Emotional {
    // A method to get a happy expression
    fn get_happy(&mut self) -> String;

    // A method to get an angry expression
    fn get_anger(&mut self) -> String;

    // A method to tell the current emotional state
    fn tell_state(&self) -> String;
}

// Definition of a struct 'HappyPerson' implementing the 'Emotional' trait
struct HappyPerson {
    // Fields of the struct
    name: String,
    state: Emotion,
}

// Implementation of the 'Emotional' trait for the 'HappyPerson' struct
impl Emotional for HappyPerson {
    // Method to get an angry expression (not yet implemented)
    fn get_anger(&mut self) -> String {
        unimplemented!()
    }

    // Method to get a happy expression
    fn get_happy(&mut self) -> String {
        format!("{} is always happy.", self.name)
    }

    // Method to tell the current emotional state (not yet implemented)
    fn tell_state(&self) -> String {
        todo!()
    }
}

// Main function where the program execution begins
fn main() {
    // Creating a 'HappyPerson' instance named 'p'
    let mut p = HappyPerson {
        name: "Takashi".to_string(),
        state: Emotion::Happy,
    };

    // Printing the result of the 'get_happy' method for the 'HappyPerson' instance
    println!("{}", p.get_happy());
}

// Define a trait 'Tweet' with three methods: tweet, tweet_twice, and shout
trait Tweet {
    // Method to perform a simple tweet
    fn tweet(&self);

    // Method to tweet twice by calling the tweet method twice
    fn tweet_twice(&self) {
        self.tweet();
        self.tweet();
    }

    // Method to shout a generic exclamation
    fn shout(&self) {
        println!("Uooooohhh!!!");
    }
}

// Define a struct 'Dove'
struct Dove;

// Define a struct 'Duck'
struct Duck;

// Implement the 'Tweet' trait for the 'Dove' struct
impl Tweet for Dove {
    // Implement the 'tweet' method for 'Dove' with a specific tweet sound
    fn tweet(&self) {
        println!("Coo!");
    }
}

// Implement the 'Tweet' trait for the 'Duck' struct
impl Tweet for Duck {
    // Implement the 'tweet' method for 'Duck' with a specific tweet sound
    fn tweet(&self) {
        println!("Quack!");
    }
}

// Main function where the program execution begins
fn main() {
    // Create an instance of 'Dove'
    let dove = Dove {};

    // Call the 'tweet' method for 'Dove'
    dove.tweet();

    // Call the 'tweet_twice' method for 'Dove'
    dove.tweet_twice();

    // Call the 'shout' method for 'Dove'
    dove.shout();

    // Create an instance of 'Duck'
    let duck = Duck {};

    // Create a vector of trait objects (dyn Tweet) containing both 'Dove' and 'Duck'
    let bird_vec: Vec<Box<dyn Tweet>> = vec![Box::new(dove), Box::new(duck)];

    // Iterate through the vector and call the 'tweet' method for each bird
    for bird in bird_vec {
        bird.tweet();
    }
}

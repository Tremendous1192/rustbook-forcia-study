// Define a module named 'module_a'
mod module_a {
    // Define a public function 'calc' within 'module_a'
    pub fn calc() {
        println!("Hello, module a!");
    }
}

// Define another module named 'module_b'
mod module_b {
    // Define a public function 'calc' within 'module_b'
    pub fn calc() {
        println!("Hello, module b!");
    }
}

// Define the main function
fn main() {
    // Import the entire 'module_a' module
    use module_a;
    // Import only the 'calc' function from 'module_b'
    use module_b::calc;

    // Call the 'calc' function from 'module_a'
    module_a::calc();
    // Call the 'calc' function from 'module_b'
    calc();
}

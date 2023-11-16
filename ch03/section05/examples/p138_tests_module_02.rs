// Public function 'add' that takes two i32 parameters and returns their sum
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

// Conditional compilation using #[cfg(test)] to include this module only during tests
#[cfg(test)]
mod tests {
    // Importing the 'add' function from the outer (super) scope
    use super::*;

    // Test function 'it_works' using the #[test] attribute
    #[test]
    fn it_works() {
        // Assert that the result of add(2, 2) is equal to 4
        assert_eq!(add(2, 2), 4);
    }
}

fn main() {}

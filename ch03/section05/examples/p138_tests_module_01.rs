// Conditional compilation using #[cfg(test)] to include this module only during tests
#[cfg(test)]
mod tests {
    // Test function 'it_works' using the #[test] attribute
    #[test]
    fn it_works() {
        // Assert that the sum of 2 and 2 is equal to 4
        assert_eq!(2 + 2, 4);
    }
}

fn main() {}

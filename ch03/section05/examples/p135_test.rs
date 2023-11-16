// Define a public function 'add' that takes two integers and returns their sum
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

// Define a test function 'test_add' using the #[test] attribute
#[test]
fn test_add() {
    // Assert that the result of add(0, 0) is equal to 0
    assert_eq!(0, add(0, 0));
    // Assert that the result of add(0, 1) is equal to 1
    assert_eq!(1, add(0, 1));
    // Assert that the result of add(1, 0) is equal to 1
    assert_eq!(1, add(1, 0));
    // Assert that the result of add(1, 1) is equal to 2
    assert_eq!(2, add(1, 1));
}

fn main() {}

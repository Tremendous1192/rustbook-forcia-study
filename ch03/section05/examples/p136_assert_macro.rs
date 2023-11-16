// Define a test function 'assert_sample' using the #[test] attribute
#[test]
fn assert_sample() {
    // Assert that the boolean expression 'true' is true
    assert!(true);

    // Assert that the boolean expression 'false' is false, providing a custom panic message
    assert!(false, "panic! value = {}", false);

    // Assert that the values true and true are equal
    assert_eq!(true, true);

    // Assert that the values true and false are not equal
    assert_ne!(true, false);

    // Assert that the values true and false are equal, providing a custom panic message
    assert_eq!(true, false, "panic! value = ({}, {})", true, false);
}

fn main() {}

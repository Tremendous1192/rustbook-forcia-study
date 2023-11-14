// Main function where the program execution begins
fn main() {
    // The following assertions are used for testing and debugging purposes.

    // assert!(expression) - Panics if the specified expression evaluates to false.
    assert!(true);

    // assert_eq!(left, right) - Panics if the left and right expressions are not equal.
    assert_eq!(1, 1);

    // assert_ne!(left, right) - Panics if the left and right expressions are equal.
    assert_ne!(1, 0);

    // debug_assert!(expression) - Similar to assert! but only active in debug builds.
    debug_assert!(false);

    // debug_assert_eq!(left, right) - Similar to assert_eq! but only active in debug builds.
    debug_assert_eq!(1, 1);

    // debug_assert_ne!(left, right) - Similar to assert_ne! but only active in debug builds.
    debug_assert_ne!(1, 0);
}

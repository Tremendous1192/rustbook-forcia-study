// Define a macro named `five_times` using the `macro_rules!` syntax
// The macro takes a single expression ($x) and multiplies it by 5
macro_rules! five_times {
    ($x:expr) => {
        5 * $x
    };
}

fn main() {
    // Use the `five_times!` macro to calculate 5 times the result of the expression (2 + 3)
    assert_eq!(25, five_times!(2 + 3));
}

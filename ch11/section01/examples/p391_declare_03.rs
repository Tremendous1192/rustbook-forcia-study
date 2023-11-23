// Define a macro named `my_vec` using the `macro_rules!` syntax
// The macro uses a repetition construct `($($x:expr),*)` to match and capture multiple expressions separated by commas
// It creates a new `Vec` containing the captured expressions using the repetition with `$(...)*`
// The double curly braces are used to create a block, ensuring that the temporary vector is correctly isolated
macro_rules! my_vec {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            // For each captured expression, push it into the temporary vector
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    // Use the `my_vec!` macro to create a vector containing the value 0
    let x = my_vec![0];
    // Print the resulting vector
    println!("{:?}", x);

    // Use the `my_vec!` macro to create a vector containing the values 0, 1, and 2
    let y = my_vec![0, 1, 2];
    // Print the resulting vector
    println!("{:?}", y);
}

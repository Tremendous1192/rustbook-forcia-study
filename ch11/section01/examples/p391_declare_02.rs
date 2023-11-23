// Define a macro named `my_vec` using the `macro_rules!` syntax
// The macro takes a single expression ($x) and creates a new `Vec` containing that expression
// The double curly braces are used to create a block, ensuring that the temporary vector is correctly isolated
macro_rules! my_vec {
    ($x:expr) => {{
        let mut temp_vec = Vec::new();
        temp_vec.push($x);
        temp_vec
    }};
}

fn main() {
    // Use the `my_vec!` macro to create a vector containing the value 0
    let x = my_vec![0];
    // Print the resulting vector
    println!("{:?}", x);
}

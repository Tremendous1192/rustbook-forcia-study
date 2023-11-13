fn main() {
    // Declare an immutable variable immut_val with the value 10
    let immut_val = 10;
    // Print the value of immut_val
    println!("{}", immut_val);

    // Declare a mutable variable mut_val with the value 20
    let mut mut_val = 20;
    // Print the value of mut_val
    println!("{}", mut_val);

    // Modify the value of mut_val by adding immut_val to it
    mut_val += immut_val;
    // Print the updated value of mut_val
    println!("{}", mut_val);

    // Declare a variable v1 of type u64 with the value 10
    let v1: u64 = 10;
    // Print the value of v1
    println!("{}", v1);

    // Declare a variable v2 of type u64 with the value 10 using a suffix
    let v2 = 10_u64;
    // Print the value of v2
    println!("{}", v2);
}

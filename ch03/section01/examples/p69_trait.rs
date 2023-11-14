// Define a struct 'A' with a single field of type i32 and derive Eq and PartialEq traits
#[derive(Eq, PartialEq)]
struct A(i32);

// Define a struct 'B' with a single field of type f32 and derive PartialEq and PartialOrd traits
#[derive(PartialEq, PartialOrd)]
struct B(f32);

// Define a unit struct 'C' and derive the Copy and Clone traits
#[derive(Copy, Clone)]
struct C;

// Define a struct 'D' and derive the Clone trait
#[derive(Clone)]
struct D;

// Define a struct 'E' and derive the Debug trait
#[derive(Debug)]
struct E;

// Define a struct 'F' and derive the Default trait
#[derive(Default)]
struct F;

// Main function where the program execution begins
fn main() {
    // Check if two instances of 'A' are equal
    println!("{:?}", A(0) == A(1));

    // Check if one instance of 'B' is greater than another
    println!("{:?}", B(1.0) > B(0.0));

    // Create an instance of 'C' and attempt to clone it into two other instances
    let c0 = C;
    let _c1 = c0;
    let _c2 = c0;

    // Create an instance of 'D' and clone it into another instance
    let d0 = D;
    let _d1 = d0.clone();

    // Print an instance of 'E' using the Debug format
    println!("{:?}", E);

    // Create an instance of 'F' using the Default trait
    let _f = F::default();
}

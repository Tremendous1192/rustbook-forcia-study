// Define a struct named 'Color' representing an RGB color with integer components
struct Color {
    r: i32,
    g: i32,
    b: i32,
}

// Main function where the program execution begins
fn main() {
    // Create an instance 'a' of the 'Color' struct with RGB values 255, 255, 255 (white)
    let a = Color {
        r: 255,
        g: 255,
        b: 255,
    };

    // Create a new variable 'b' and assign the value of 'a' to it (structs are copied)
    let b = a;

    // Print the RGB components of the 'b' instance
    println!("{}, {}, {}", b.r, b.g, b.b);
}

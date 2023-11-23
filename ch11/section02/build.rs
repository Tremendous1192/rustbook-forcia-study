// Main function
fn main() {
    // Use the `cc` crate to build the "hello" executable from the "src/hello.c" source file
    cc::Build::new().file("src/hello.c").compile("hello");
}

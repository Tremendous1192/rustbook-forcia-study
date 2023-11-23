// External C function declaration
extern "C" {
    fn c_fib(n: u32) -> u32;
}

fn main() {
    // Call to external C function to calculate Fibonacci sequence
    unsafe {
        println!("{}", c_fib(10));
    }
}

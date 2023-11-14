fn main() {
    fn f(x: usize) -> &'static str {
        match x {
            n if n * n % 3 == 0 => "3n",
            n if n * n % 3 == 1 => "3n + 1 or 3n + 2",
            _ => unreachable!(),
        }
    }

    println!("{}", f(1));
}

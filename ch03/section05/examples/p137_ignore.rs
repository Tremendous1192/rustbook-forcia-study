// このファイルのテストを実行する場合、下記のコマンドを入力すること
// cargo test --example p137_ignore -- --ignored

// Define a public function 'add' that takes two i32 parameters and returns their sum
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

// Define a test function 'test_add_ignored' using the #[test] attribute, and mark it as ignored with #[ignore]
#[test]
#[ignore]
fn test_add_ignored() {
    // Assert that the sum of -1 and -1 is equal to -2
    assert_eq!(-2, add(-1, -1));
}

fn main() {}

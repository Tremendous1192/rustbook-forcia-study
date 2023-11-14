fn main() {
    // Result型を使って、エラーハンドリングを示します。Result<i32, String>型のresult変数を宣言し、Ok(200)で初期化します。
    let result: Result<i32, String> = Ok(200);

    // if let式を使用して、Result型をパターンマッチングします。
    if let Ok(code) = result {
        // マッチが成功した場合、code変数に中の値をバインドし、その値を表示します。
        println!("code: {}", code);
    }
}

fn main() {
    // Result型を使って、エラーハンドリングを示します。Result<i32, String>型のresult変数を宣言し、Ok(200)で初期化します。
    let result: Result<i32, String> = Ok(200);

    // match式を使用してResult型をパターンマッチングします。
    match result {
        // マッチした結果がOkの場合、code変数に中の値をバインドし、その値を表示します。
        Ok(code) => println!("code: {}", code),
        // マッチした結果がErrの場合、err変数に中の値をバインドし、その値を表示します。
        Err(err) => println!("Err: {}", err),
    }
}

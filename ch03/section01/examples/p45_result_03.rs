fn main() {
    // Result型を使ってエラーハンドリングを示します。Result<i32, String>型のresult変数を宣言し、Ok(200)で初期化します。
    let result: Result<i32, String> = Ok(200);

    // unwrap_orメソッドを使用して、ResultがOkの場合に内部の値を取得し、それを表示します。エラーの場合はデフォルトの値(-1)が表示されます。
    println!("code: {}", result.unwrap_or(-1));

    // 別のResult変数をErrで初期化します。
    let result: Result<i32, String> = Err("error".to_string());

    // unwrap_orメソッドを使用して、ResultがErrの場合にデフォルトの値(-1)を取得し、それを表示します。
    println!("code: {}", result.unwrap_or(-1));
}

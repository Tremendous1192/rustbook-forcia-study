fn main() {
    // 関数funcはi32の引数を受け取り、Result<i32, String>を返す関数です。
    fn func(code: i32) -> Result<i32, String> {
        // 引数codeを表示します。
        println!("code: {}", code);
        // Ok(100)を返します。常に成功する結果を表します。
        Ok(100)
    }

    // Result<i32, String>型のresult変数を宣言し、Ok(200)で初期化します。
    let result: Result<i32, String> = Ok(200);

    // and_thenメソッドを使用して、resultにfunc関数を適用し、新しいResult型のnext_resultを作成します。
    let next_result = result.and_then(func);

    // 別のResult変数をErrで初期化します。
    let result: Result<i32, String> = Err("error".to_string());

    // and_thenメソッドを使用して、resultにfunc関数を適用し、新しいResult型のnext_resultを作成します。ただし、エラーが起きるため、func関数は呼ばれません。
    let next_result = result.and_then(func);
}

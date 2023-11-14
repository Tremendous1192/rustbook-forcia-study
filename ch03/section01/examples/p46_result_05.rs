fn main() {
    // error_handling関数はResult<i32, String>を引数に受け取り、新しいResult<i32, String>を返します。
    fn error_handling(result: Result<i32, String>) -> Result<i32, String> {
        // ?演算子を使用してResult内の値を取り出し、エラーがあれば早期リターンします。
        let code = result?;
        // codeの値を表示します。
        println!("code: {}", code);
        // 常に成功するOk(100)を返します。
        Ok(100)
    }

    // Result<i32, String>型のresult変数を宣言し、Ok(200)で初期化します。
    let result: Result<i32, String> = Ok(200);
    // error_handling関数にresultを渡し、新しいResult型の結果を無視します（_でマッチさせていません）。
    let _ = error_handling(result);

    // 別のResult変数をErrで初期化します。
    let result: Result<i32, String> = Err("error".to_string());
    // error_handling関数にresultを渡し、新しいResult型の結果を無視します。
    let _ = error_handling(result);
}

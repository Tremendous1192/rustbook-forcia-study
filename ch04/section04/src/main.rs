// RPN (逆ポーランド記法) 計算機プログラム
// 指定したテキストファイルの各行を逆ポーランド記法に基づいて計算する
// 実行コマンド
// cargo run -- input.txt

// 使用ライブラリのインポート
use anyhow::{bail, ensure, Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

// RpnCalculator構造体の定義
struct RpnCalculator(bool);

impl RpnCalculator {
    // RpnCalculatorの新しいインスタンスを作成するコンストラクタ関数
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    // 式を評価するメソッド
    pub fn eval(&self, formula: &str) -> Result<i32> {
        // 式をトークンに分割し、逆順に配置
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        // 内部で使用するeval_innerメソッドを呼び出す
        self.eval_inner(&mut tokens)
    }

    // 内部で使用する評価メソッド
    fn eval_inner(&self, tokens: &mut Vec<&str>) -> Result<i32> {
        let mut stack = Vec::new();
        let mut pos = 0;

        // トークンが存在する限り繰り返す
        while let Some(token) = tokens.pop() {
            pos += 1;

            // トークンを整数に変換できればスタックにプッシュ
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                // トークンが演算子の場合
                let y = stack.pop().context(format!("invalid syntax at {}", pos))?;
                let x = stack.pop().context(format!("invalid syntax at {}", pos))?;
                // 演算を実行し結果をスタックにプッシュ
                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => bail!("invalid token at {}", pos),
                };

                stack.push(res);
            }

            // デバッグモード時にトークンとスタックの状態を表示
            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }

        // スタックの長さが1であることを確認し、結果を返す
        ensure!(stack.len() == 1, "invalid syntax");

        Ok(stack[0])
    }
}

// コマンドライン引数を取得するためのOpts構造体の定義
#[derive(Parser, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "Your name",
    about = "Super awesome sample RPN calculator"
)]
struct Opts {
    #[clap(short, long)]
    verbose: bool,

    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

// メイン関数
fn main() -> Result<()> {
    // コマンドライン引数をパースしてOpts構造体を作成
    let opts = Opts::parse();

    // フォーミュラファイルが指定されている場合
    if let Some(path) = opts.formula_file {
        // ファイルを開く
        let f = File::open(&path)?;

        // バッファリーダーを作成し、計算を実行するrun関数を呼び出す
        let reader = BufReader::new(f);
        run(reader, opts.verbose)
    } else {
        // フォーミュラファイルが指定されていない場合は標準入力を使用
        let stdin = stdin();
        let reader = stdin.lock();
        // 計算を実行するrun関数を呼び出す
        run(reader, opts.verbose)
    }
}

// run関数: ファイルまたは標準入力から読み取った行ごとに計算を実行する
fn run<R: BufRead>(reader: R, verbose: bool) -> Result<()> {
    // RpnCalculatorの新しいインスタンスを作成
    let calc = RpnCalculator::new(verbose);

    // 読み取った各行に対して計算を実行
    for line in reader.lines() {
        let line = line?;

        // calc.evalメソッドを呼び出し、計算結果を出力
        match calc.eval(&line) {
            Ok(answer) => println!("{} = {}", line, answer),
            Err(e) => eprintln!("{:#?}", e),
        }
    }

    Ok(())
}

// テストモジュール
#[cfg(test)]
mod tests {
    use super::*;

    // 正常なテストケース
    #[test]
    fn test_ok() {
        let calc = RpnCalculator::new(false);
        assert_eq!(calc.eval("5").unwrap(), 5);
        assert_eq!(calc.eval("50").unwrap(), 50);
        assert_eq!(calc.eval("-50").unwrap(), -50);

        assert_eq!(calc.eval("2 3 +").unwrap(), 5);
        assert_eq!(calc.eval("2 3 *").unwrap(), 6);
        assert_eq!(calc.eval("2 3 -").unwrap(), -1);
        assert_eq!(calc.eval("2 3 /").unwrap(), 0);
        assert_eq!(calc.eval("2 3 %").unwrap(), 2);
    }

    // エラーが発生するテストケース
    #[test]
    fn test_ng() {
        let calc = RpnCalculator::new(false);
        assert!(calc.eval("").is_err());
        assert!(calc.eval("1 1 1 +").is_err());
        assert!(calc.eval("+ 1 1").is_err());
    }
}

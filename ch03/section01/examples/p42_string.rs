fn main() {
    // 新しいString型の変数s1を作成し、文字列 "Hello, World!" を格納します。
    let s1: String = String::from("Hello, World!");

    // s1の参照をs2に格納します。s2は文字列のスライス（&str）です。
    let s2: &str = &s1;

    // s2（文字列のスライス）から新しいString型の変数s3を生成します。
    let s3: String = s2.to_string();

    // s1の内容を出力します。s1は依然として有効です。
    println!("{}", s1);

    // s2の内容を出力します。s2はs1の参照です。
    println!("{}", s2);

    // s3の内容を出力します。s3はs2から生成された新しいStringです。
    println!("{}", s3);
}

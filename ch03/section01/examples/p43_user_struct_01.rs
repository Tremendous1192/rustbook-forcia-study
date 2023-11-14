fn main() {
    // Personという名前の構造体を宣言します。この構造体はname（文字列）とage（非負整数）の2つのフィールドを持ちます。
    struct Person {
        name: String,
        age: u32,
    }

    // Person構造体に対するDisplayトレイトの実装を行います。このトレイトを実装することで、フォーマットをカスタマイズできます。
    impl core::fmt::Display for Person {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            // フォーマット文字列を指定し、nameとageの値を埋め込んでフォーマットします。
            write!(f, "(name, age) = ({}, {})", self.name, self.age)?;
            Ok(())
        }
    }

    // Person構造体の新しいインスタンスpを生成します。nameフィールドに"John"、ageフィールドに8を設定します。
    let p = Person {
        name: String::from("John"),
        age: 8,
    };

    // Displayトレイトを実装したため、{}でpを出力することができます。カスタマイズされたフォーマットで表示されます。
    println!("{}", p);

    // 構造体のフィールドnameを個別に出力します。
    println!("name = {}", p.name);

    // 構造体のフィールドageを個別に出力します。
    println!("age = {}", p.age);
}

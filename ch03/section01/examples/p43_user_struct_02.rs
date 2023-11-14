fn main() {
    // Debugトレイトを自動導出するEventという名前の列挙型を定義します。列挙型は異なる種類のイベントを表します。
    #[derive(Debug)]
    enum Event {
        Quit,                         // Quitイベントは引数を持たない簡単な列挙型です。
        KeyDown(u8),                  // KeyDownイベントはu8型の引数を持ち、キーの値を表します。
        MouseDown { x: i32, y: i32 }, // MouseDownイベントはxとyという名前のフィールドを持つ構造体です。
    }

    // 3つの異なるイベントインスタンスを生成します。
    let e1 = Event::Quit; // Quitイベントのインスタンスを作成します。
    let e2 = Event::KeyDown(10); // KeyDownイベントのインスタンスを作成し、キーの値10を指定します。
    let e3 = Event::MouseDown { x: 10, y: 10 }; // MouseDownイベントのインスタンスを作成し、xとyの値を指定します。

    // Debugトレイトを導出しているため、{:?}フォーマット指定子を使用して各イベントをデバッグ表示します。
    println!("{:?}", e1); // Quitイベントを表示します。
    println!("{:?}", e2); // KeyDownイベントを表示し、関連するキーの値も表示します。
    println!("{:?}", e3); // MouseDownイベントを表示し、xとyの値も表示します。
}

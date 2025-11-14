use ferris_says::say; // 外部ライブラリの 'say' 関数を使うことを宣言
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout(); // 標準出力を取得
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    // 出力先（バッファ）を用意
    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}
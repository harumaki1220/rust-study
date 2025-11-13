fn main() {
    let number = 7;

    if number < 5 {
        println!("条件はtrueです（numberは5より小さい）");
    } else {
        println!("条件はfalseです（numberは5以上です）");
    }

    let condition = true;
    let result = if condition {
        5 // i32(整数)
    } else {
        10 // &str(文字列)にできない。TypeScriptでいうところの let result: number | string;はエラーになる
    };

    println!("'if'式の結果は: {}", result); // 
}
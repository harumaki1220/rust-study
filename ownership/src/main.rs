fn main() {
    println!("--- i32 (Copy) の場合 ---");
    let x = 5; // i32は「Copy」トレイトを持つ
    let y = x; // 値がコピーされる（ムーブではない）

    // xもyも両方使える
    println!("x = {}, y = {}", x, y);

    println!("--------------------------");
    println!("--- String (Move) の場合 ---");
    let s1 = String::from("hello"); // s1がデータの所有権を持つ
    let s2 = s1; // s1からs2へ所有権が「ムーブ」する

    println!("s2 = {}", s2);
 

    println!("--------------------------");
    println!("--- Vec (Move) の場合 ---");
    let v1 = vec![1, 2, 3]; // v1がベクタの所有権を持つ
    let v2 = v1; // v1からv2へ所有権が「ムーブ」する

    println!("v2 = {:?}", v2); // Vecの表示は :? が便利


    println!("--------------------------");
    println!("--- Box (Move) の場合 ---");
    let b1 = Box::new(5); // b1がBoxの所有権を持つ
    let b2 = b1; // b1からb2へ所有権が「ムーブ」する
    println!("b2 = {}", b2);

    
    println!("--------------------------");
    println!("--- 関数とムーブ ---");
    let s_main = String::from("this is a test");
    let len = calculate_length(&s_main);

    println!("'{}' の長さは {} です。", s_main, len);
}

fn calculate_length(s: &String) -> usize { // &String は「String 型への参照」
    let length = s.len(); // .len() は文字列のバイト数を返す
    length
}
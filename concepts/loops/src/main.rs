fn main() {
    print_one_to_five();
    println!("--------------------");
    do_countdown();
}


fn print_one_to_five() {
    println!("課題1: 1から5まで表示");
    for i in 1..=5 {
        println!("{}", i);
    }
}

fn do_countdown() {
    println!("課題2: カウントダウン");
    for a in (1..=3).rev() {
        println!("for: {}!", a);
    }
    println!("for: 発射！");
}
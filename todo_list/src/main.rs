#[derive(Debug)]
struct Task {
    name: String,
    completed: bool,
}
fn main() {
    println!("シンプルなToDoリストへようこそ!");

    let mut tasks: Vec<Task> = Vec::new();
    loop {
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input);

        let command: &str = input.trim();

        if command == "list" {
            // リスト表示の処理
        } else if command.starts_with("add ") {
        } else if command == "quit" {
            break;
        } else {
            // それ以外のコマンドが入力された時の処理
        }
    }
}

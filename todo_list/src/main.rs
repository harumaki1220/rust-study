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
        std::io::stdin()
            .read_line(&mut input)
            .expect("入力の読み取りに失敗しました");

        let command: &str = input.trim();

        if command == "list" {
            if tasks.is_empty() {
                println!("タスクはありません。");
            } else {
                println!("--- ToDoリスト ---");

                for (i, task) in tasks.iter().enumerate() {
                    let status = if task.completed { "[x]" } else { "[ ]" };
                    println!("{}. {} {}", i + 1, status, task.name)
                }
                println!("------------------");
            }
        } else if command.starts_with("add ") {
            if let Some(task_name_str) = command.strip_prefix("add ") {
                let new_task = Task {
                    name: task_name_str.to_string(),
                    completed: false,
                };
                tasks.push(new_task);
                println!("タスクを追加しました: {}", task_name_str);
            } else {
                println!("タスク名を入力してください。例: add 洗濯");
            }
        } else if command == "quit" {
            break;
        } else {
            println!("不明なコマンドです。 (add, list, quit が使えます)");
        }
    }
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    fn print_username(&self) {
        println!("Username is: {}", self.username);
    }
    fn increment_sign_in(&mut self) {
        self.sign_in_count += 1;
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someuser123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("User1のEメール: {}", user1.email);
    user1.email = String::from("another@example.com");
    println!("User1の新しいEメール: {}", user1.email);
    println!("User1の全情報: {:#?}", user1);

    user1.print_username(); // 

    user1.increment_sign_in();
    println!("サインイン回数: {}", user1.sign_in_count); //
}
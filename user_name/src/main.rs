use std::io;

fn main() {
    println!("Enter username");
    let mut user_name = String::new();
    io::stdin()
        .read_line(&mut user_name)
        .expect("Invalid entry");

    println!("Welcome {}", &user_name);
}
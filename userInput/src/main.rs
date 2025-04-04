use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Helloworld");

    let input: usize = input.trim().parse().expect("Number");

    println!("Input keyin {input}");
}
mod lib;

use lib::*;
use std::io;

pub fn main() {
    println!("Welcome to ze calculator...");

    print!("Enter first number :");
    let mut first_number : u16 = 0;
    io::stdin()
        .read_line(&mut first_number.to_string())
        .expect("Invalid number");

    print!("Enter Operation :");
    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation.to_string())
        .expect("Invalid operation");

    print!("Enter second number :");
    let mut second_number : u16 = 0;
    io::stdin()
        .read_line(&mut second_number.to_string())
        .expect("Invalid number"); 
}
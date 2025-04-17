pub fn add(num1: u16, num2:u16) -> u16 {
    &num1 + &num2
}

pub fn subtract(num1: u16, num2:u16) -> u16 {
    &num1 - &num2
}

pub fn multiply(num1: u16, num2:u16) -> u16 {
    &num1 * &num2
}

pub fn divide(num1: u16, num2:u16) -> f64 {
    (&num1 / &num2) as f64
}

pub fn get_result(num1: &str, num2: &str, operand: &str) -> String {
    match operand{
        "+" => add(num1.parse::<u16>().unwrap(), num2.parse::<u16>().unwrap()).to_string(),
        "-" => subtract(num1.parse::<u16>().unwrap(), num2.parse::<u16>().unwrap()).to_string(),
        "*" => multiply(num1.parse::<u16>().unwrap(), num2.parse::<u16>().unwrap()).to_string(),
        "/" => divide(num1.parse::<u16>().unwrap(), num2.parse::<u16>().unwrap()).to_string(),
        &_ => "Invalid Input".to_string()
    }
}
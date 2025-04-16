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
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const LIMIT: u32 = 10000000;
    println!("This is the limit {LIMIT}");

    let shadowed_value = 7;
    let shadowed_value = shadowed_value + 2;

    {
        let shadowed_value = shadowed_value * 2;
        println!("This is the value of inner scope shadowedValue: {shadowed_value}");
    }

    println!("This is the value of main function scope shadowedValue : {shadowed_value}")
}
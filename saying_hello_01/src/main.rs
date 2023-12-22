/**
*  Problem Statement:
*
*  The “Hello, World” program is the first program you learn
*  to write in many languages, but it doesn’t involve any input.
*
*  So create a program that prompts for your name and prints
*  a greeting using your name.
*
*  Example Output
*  What is your name? Brian
*  Hello, Brian, nice to meet you!
*
*  Constraint
*      • Keep the input, string concatenation, and output separate
**/
use std::io::stdin;

fn get_name() -> String {
    let mut name = String::new();

    stdin().read_line(&mut name).expect("failed to read line.");

    name
}

pub fn greeting(name: String) -> String {
    format!("Hello, {}, nice to meet you!", name)
}

fn main() {
    let name = get_name();

    println!("{}", greeting(name));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_works() {
        let greeting = greeting("Jeff".to_string());
        assert!(greeting.contains("Jeff"));
    }
}

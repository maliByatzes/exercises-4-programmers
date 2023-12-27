// You’ll often write programs that deal with numbers. And
// depending on the programming language you use, you’ll
// have to convert the inputs you get to numerical data types.
// Write a program that prompts for two numbers. Print the
// sum, difference, product, and quotient of those numbers as
// shown in the example output:
//
// Example Output
// What is the first number? 10
// What is the second number? 5
// 10 + 5 = 15
// 10 - 5 = 5
// 10 * 5 = 50
// 10 / 5 = 2
//
// Constraints
// • Values coming from users will be strings. Ensure that
// you convert these values to numbers before doing the
// math.
// • Keep the inputs and outputs separate from the numerical
// conversions and other processing.
// • Generate a single output statement with line breaks in
// the appropriate spots.

use std::io::stdin;

fn get_number(pos: &str) -> i32 {
    println!("What is the {} number?", pos);

    let mut number = String::new();
    stdin()
        .read_line(&mut number)
        .expect("failed to read number");

    let number: i32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Enter a number to continue.");
            0
        }
    };

    number
}

fn main() {
    let number1 = get_number("first");
    let number2 = get_number("second");

    println!("{} + {} = {}", number1, number2, number1 + number2);
    println!("{} - {} = {}", number1, number2, number1 - number2);
    println!("{} * {} = {}", number1, number2, number1 * number2);
    println!("{} / {} = {}", number1, number2, number1 / number2);
}

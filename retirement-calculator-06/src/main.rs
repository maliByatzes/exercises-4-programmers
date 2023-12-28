// Your computer knows what the current yearis, which means
// you can incorporate that into your programs. You just have
// to figure out how your programming language can provide
// you with that information.
//
// Create a program that determines how many years you have
// left until retirement and the year you can retire. It should
// prompt for your current age and the age you want to retire
// and display the output as shown in the example that follows.
//
// Example Output
// What is your current age? 25
// At what age would you like to retire? 65
// You have 40 years left until you can retire.
// It's 2015, so you can retire in 2055.
//
// Constraints
// • Again, be sure to convert the input to numerical data
// before doing any math.
// • Don’t hard-code the current year into your program.
// Get it from the system time via your programming language.

use std::io::stdin;

use chrono::{Datelike, Local};

fn main() {
    // Get user's current age
    let mut current_age = String::new();
    println!("What is your current age?");
    stdin()
        .read_line(&mut current_age)
        .expect("failed to read age");
    let current_age: i32 = current_age.trim().parse().expect("failed to parse age");

    // Get the age of retirement
    let mut retire_age = String::new();
    println!("At what age would you like to retire?");
    stdin()
        .read_line(&mut retire_age)
        .expect("failed to read age");
    let retire_age: i32 = retire_age.trim().parse().expect("failed to parse age");

    // Print years left till retirement
    if current_age < retire_age {
        println!(
            "You have {} years left until you can retire.",
            retire_age - current_age
        );
    } else {
        println!("You already reached your retirement age.");
    }

    // Get current year
    let current_year = Local::now().year();

    println!(
        "It's {}, so you can retire in {}",
        current_year,
        current_year + (retire_age - current_age)
    );
}

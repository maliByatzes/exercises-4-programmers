// Mad libs are a simple game where you create a story template with blanks for words. You, or another player, then
// construct a list of words and place them into the story, creating an often silly or funny story as a result.
//
// Create a simple mad-lib program that prompts for a noun,
// a verb, an adverb, and an adjective and injects those into a
// story that you create.
//
// Example Output
// Enter a noun: dog
// Enter a verb: walk
// Enter an adjective: blue
// Enter an adverb: quickly
// Do you walk your blue dog quickly? That's hilarious!
//
// Constraints
// • Use a single output statement for this program.
// • If your language supports string interpolation or string
// substitution, use it to build up the output.

use std::io::stdin;

fn create_story() -> String {
    let mut noun = String::new();

    println!("Enter a noun:");
    stdin().read_line(&mut noun).expect("failed to read noun");

    let mut verb = String::new();

    println!("Enter a verb:");
    stdin().read_line(&mut verb).expect("failed to read verb");

    let mut adjective = String::new();

    println!("Enter an adjective:");
    stdin()
        .read_line(&mut adjective)
        .expect("failed to read adjective");

    let mut adverb = String::new();

    println!("Enter an adverb:");
    stdin()
        .read_line(&mut adverb)
        .expect("failed to read adverb");

    format!(
        "The {} {} {} {} through the {} forest.",
        adjective.trim(),
        noun.trim(),
        adverb.trim(),
        verb.trim(),
        adjective.trim()
    )
}

fn main() {
    println!("{}", create_story());
}

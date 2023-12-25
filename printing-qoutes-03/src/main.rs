use std::collections::HashMap;

/**
// Get the qoute and the author from the user
fn get_qoute_and_author() -> String {
    let mut qoute = String::new();
    let mut author = String::new();

    println!("What is the qoute?");
    stdin()
        .read_line(&mut qoute)
        .expect("failed to read the qoute");

    println!("What said it?");
    stdin()
        .read_line(&mut author)
        .expect("failed to read the author");

    format!("{} says, \"{}\"", author.trim(), qoute.trim())
}**/

fn main() {
    // Using data structures to store the qoutes instead of proompting the user
    let mut qoutes = HashMap::new();

    qoutes.insert(
        "These aren't the droids you are looking for.",
        "Obi-Wan Kenobi",
    );
    qoutes.insert("Knowledge is power.", "Sir Francis Bacon");
    qoutes.insert("All that glitters is not gold.", "William Shakespeare");
    qoutes.insert("Eigthy percent of success is showing up.", "Woody Allen");

    // Display all qoutes
    for (qoute, author) in qoutes {
        println!("{} says, \"{}\"", author, qoute)
    }
}

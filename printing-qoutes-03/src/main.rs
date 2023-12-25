use std::io::stdin;

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
}

fn main() {
    println!("{}", get_qoute_and_author());
}

// Create a program that prompts for an input string and
// displays output that shows the input string and the number of
// characters the string contains.
//
// Example Output
// What is the input string? Homer
// Homer has 5 characters.
//
// Constraints
// • Be sure the output contains the original string.
// • Use a single output statement to construct the output.
// • Use a built-in function of the programming language to
//   determine the length of the string.

use std::io::stdin;

fn get_word() -> String {
    let mut word = String::new();

    loop {
        println!("What is the input string:");
        stdin().read_line(&mut word).expect("failed to read word");
        word = word.trim().to_owned();

        if word == "" {
            println!("You must enter something to continue.")
        } else {
            break;
        }
    }

    word
}

pub fn count_word(word: &String) -> i32 {
    word.chars().count() as i32
}

fn main() {
    let word = get_word();

    println!("{} has {} characters.", &word, count_word(&word));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_word_test() {
        let chars = count_word(&"hi".to_string());

        assert_eq!(2, chars);
    }

    #[test]
    fn long_word_test() {
        let chars = count_word(&"documentation".to_string());

        assert_eq!(13, chars);
    }

    #[test]
    fn even_longer_word_test() {
        let chars = count_word(&"documentation of this project".to_string());

        assert_eq!(29, chars);
    }
}

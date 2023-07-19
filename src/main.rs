use std::io;
use std::str;
use std::collections::HashSet;

use rand::prelude::SliceRandom;
use unicode_segmentation::UnicodeSegmentation;


fn main() {
    let word_to_guess: &str = get_random_word();

    let mut has_won: bool = false;
    let _answer: Vec<&str>;

    let mut picked_letters: HashSet<String> = HashSet::new();
    while !has_won {
        let input_letter = prompt_for_letter(&picked_letters);
        if input_letter.is_ok() {
            picked_letters.insert(input_letter.unwrap());
        }
        has_won = show_matches(word_to_guess, picked_letters.clone());
    }

    let count = picked_letters.len();
    println!("Bien joué champion ! En {count} coup(s) !");
}

fn get_random_word() -> &'static str {
    let deck = ["coucou", "kayak", "dinosaure"];

    return deck.choose(&mut rand::thread_rng()).copied().unwrap();
}

fn prompt_for_letter(
    picked_letters: &HashSet<String>
) -> Result<String, &'static str> {
    println!("Try a letter: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let letter: &str = input.graphemes(true).collect::<Vec<&str>>()[0];
    if picked_letters.get(letter).is_some() {
        return Err("Letter \"{letter}\" already tried");
    }


    return Ok(letter.to_owned());
}

fn show_matches(
    word: &str,
    picked_letters: HashSet<String>
) -> bool {
    let letters = word.graphemes(true).collect::<Vec<&str>>();

    let fully_matches = letters.iter().enumerate().fold(
        true,
        |mut acc, (i, x)| -> bool {
            if picked_letters.get(*x).is_some() {
                print!("{x}");
            } else {
                print!("{}", if i % 2 == 0 { "▓" } else { "▒" } );
                acc = false;
            }
            return acc;
        }
    );

    print!("\t[");
    for letter in &picked_letters {
        print!("{letter},");
    }
    println!("]");
    return fully_matches;
}

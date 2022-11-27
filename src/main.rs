use std::fs;
use std::io;
use std::str::pattern::Pattern;

use rand::seq::SliceRandom;

// enum Difficulty {
// Hard,
// Normal,
// Easy,
// }

struct WordToGuess {
    word: String,
    length: usize,
    letters_to_guess: Vec<char>,
}

struct Player {
    guess: String,
    attempts: u8,
    guessed_letters: Vec<char>,
    preview: Vec<char>,
    // difficulty: Option<Difficulty>,
}

enum GuessResult {
    Word(String),
    Letter(char),
}

impl Player {
    fn new() -> Player {
        Player {
            guess: String::new(),
            attempts: 0,
            guessed_letters: Vec::new(),
            preview: Vec::new(),
            // difficulty: None,
        }
    }
    fn compare(&mut self, word: &String) -> Result<GuessResult, ()> {
        match self.guess.trim() == word {
            true => Ok(GuessResult::Word(self.guess.clone())),
            false => match self.guess.trim().len() {
                1 => match word
                    .chars()
                    .collect::<Vec<char>>()
                    .contains(&self.guess.chars().nth(0).unwrap())
                {
                    true => {
                        self.guessed_letters
                            .push(self.guess.chars().nth(0).unwrap());
                        Ok(GuessResult::Letter(self.guess.chars().nth(0).unwrap()))
                    }

                    false => Err(()),
                },
                _ => Err(()),
            },
        }
    }
    fn guess(&mut self) -> Result<(), ()> {
        println!("guessed letters: {:?}", self.guessed_letters);
        self.attempts += 1;
        self.guess = String::new();
        match io::stdin().read_line(&mut self.guess) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}

fn main() {
    match fs::read_to_string("words") {
        Err(_) => println!("could not get word list"),
        Ok(words) => {
            match words
                .split("\n")
                .collect::<Vec<_>>()
                .choose(&mut rand::thread_rng())
            {
                None => todo!(),
                Some(word) => {
                    let word_to_guess: WordToGuess = WordToGuess {
                        word: word.to_string(),
                        length: word.len(),
                        letters_to_guess: word.chars().collect(),
                    };
                    let mut player: Player = Player::new();
                    println!("to guess: {:?}", word_to_guess.letters_to_guess);
                    for _ in word_to_guess.word.chars() {
                        player.preview.push('_');
                    }

                    loop {
                        println!("preview: {:?}", player.preview);
                        match player.guess() {
                            Err(_) => todo!(),
                            Ok(_) => {
                                match player.compare(&word_to_guess.word) {
                                    Err(_) => println!("wrong"),
                                    Ok(GuessResult::Letter(letter)) => {
                                        println!("correct letter: {letter}")
                                    }
                                    Ok(GuessResult::Word(word)) => {
                                        println!("correct word: {word}");
                                        break;
                                    }
                                };
                            }
                        };
                    }
                }
            }
        }
    }
}

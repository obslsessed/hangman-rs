use std::io;

// enum Difficulty {
// Hard,
// Normal,
// Easy,
// }

struct Player {
    guess: String,
    attempts: u8,
    // difficulty: Option<Difficulty>,
}

impl Player {
    fn new() -> Player {
        Player {
            guess: String::new(),
            attempts: 0,
            // difficulty: None,
        }
    }
    fn compare(&mut self, word: &String) -> Result<(), ()> {
        match self.guess.trim() == word {
            true => Ok(()),
            false => Err(()),
        }
    }
    fn guess(&mut self) -> Result<(), ()> {
        self.attempts += 1;
        self.guess = String::new();
        match io::stdin().read_line(&mut self.guess) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}

fn main() {
    let word: String = String::from("snake");
    let mut player: Player = Player::new();

    loop {
        // match io::stdin().read_line(&mut player.guess) {
        match player.guess() {
            Err(_) => todo!(),
            Ok(_) => {
                match player.compare(&word) {
                    Err(_) => println!("wrong"),
                    Ok(_) => break,
                };
            }
        };
    }
}

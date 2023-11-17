use std::io;

fn main() {
    let mut game: Game = Game::new(String::from("potato"));
    println!(
        "word: {}, max guesses: {}, lives: {}",
        game.word, game.letters_left_to_guess, game.lives
    );

    // get player guess
    let mut current_guess: String = String::new();
    current_guess = guess(&mut current_guess);

    // compare the guess to the word
    game.processing(current_guess);
}

struct Game {
    word: String,
    letters_left_to_guess: u8,
    lives: u8,
    player_guess: String,
}

impl Game {
    fn new(word: String) -> Game {
        let length: u8 = word.len() as u8;
        Game {
            word,
            letters_left_to_guess: length,
            lives: 6,
            player_guess: String::new(),
        }
    }

    fn processing(&self, guess: String) {
        if self.word.contains(guess) {
            println!("the word: {} contains {}", self.word, guess);
        } else {
            println!("the word: {} does not contain {}", self.word, guess);
        }
    }
}

fn guess(player_guess: &mut String) -> &str {
    let guess: &str = match io::stdin().read_line(player_guess) {
        Ok(_) => {
            let output: &str = player_guess.trim();
            output
        }
        Err(error) => panic!("{} was the error", error),
    };
    guess
}

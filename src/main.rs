use std::io;

fn main() {
    let game: Game = Game::new(String::from("potato"));
    println!(
        "word: {}, max guesses: {}, lives: {}",
        game.word, game.letters_left_to_guess, game.lives
    );

    // get player guess
    let mut player_input = String::new();
    let guess: &str = match io::stdin().read_line(&mut player_input) {
        Ok(_) => {
            let output: &str = player_input.trim();
            output
        }
        Err(error) => panic!("{} was the error", error),
    };

    // compare the guess to the word
    if game.word.contains(guess) {
        println!("the word: {} contains {}", game.word, guess);
    } else {
        println!("the word: {} does not contain {}", game.word, guess);
    }
}

struct Game {
    word: String,
    letters_left_to_guess: u8,
    lives: u8,
}

impl Game {
    fn new(word: String) -> Game {
        let length: u8 = word.len() as u8;
        Game {
            word,
            letters_left_to_guess: length,
            lives: 6,
        }
    }
}

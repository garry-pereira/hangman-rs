use std::io;

pub struct Game {
    word: String,
    letters_left_to_guess: u8,
    lives: u8,
}

impl Game {
    pub fn new(word: String) -> Game {
        let length: u8 = word.len() as u8;
        Game {
            word,
            letters_left_to_guess: length,
            lives: 6,
        }
    }

    pub fn guess(guess: &mut String) -> &str {
        let guess_output: &str = match io::stdin().read_line(guess) {
            Ok(_) => {
                let output: &str = guess.trim();
                output
            }
            Err(error) => panic!("{} was the error", error),
        };
        guess_output
    }

    pub fn check(&self, guess: &str) {
        if self.word.contains(guess) {
            println!("word: {} contains guess: {}", self.word, guess);
        } else {
            println!("word: {} does NOT contain guess: {} ", self.word, guess);
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        println!("hello");
    }
}

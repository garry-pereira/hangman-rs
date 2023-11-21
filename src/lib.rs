use std::io;

#[allow(dead_code)]
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

    pub fn accept_guess(guess: &mut String) -> &str {
        loop {
            guess.truncate(0);
            io::stdin()
                .read_line(guess)
                .expect("failed to accept input");

            print!("your guess was: {}", guess);

            if guess.trim().len() == 1 {
                break guess.trim();
            } else {
                println!("guess 1 letter at a time");
            }
        }
    }

    pub fn check(&self, guess: &str) {
        if self.word.contains(guess) {
            println!("your guess: '{}' success", guess);
        } else {
            println!("your guess: '{}' fail", guess);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn accepts_guess() {
        let mut test: String = String::new();
        assert_ne!(0, Game::accept_guess(&mut test).len())
    }
}

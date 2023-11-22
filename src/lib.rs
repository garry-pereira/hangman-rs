use std::io;

pub struct Game {
    word: String,
    letters_left_to_guess: usize,
    guessing_word: String,
    lives: u8,
    guess: String,
    incorrectly_guessed_chars: String,
    correctly_guessed_chars: String,
}

impl Game {
    pub fn new(word: String) -> Game {
        let length: usize = word.len();
        let guess: String = String::new();
        let incorrectly_guessed_chars: String = String::new();
        let guessing_word: String = "_".repeat(length);
        let correctly_guessed_chars: String = String::new();

        Game {
            word,
            letters_left_to_guess: length,
            guessing_word,
            lives: 6,
            guess,
            incorrectly_guessed_chars,
            correctly_guessed_chars,
        }
    }

    pub fn update_game_state(&mut self) {
        // show the word they have to guess
        // iterate over the actual word
        let result: String = self
            .word
            .chars()
            .zip(self.guessing_word.chars())
            .map(|(c, u)| {
                if self.correctly_guessed_chars.contains(c) {
                    c
                } else {
                    u
                }
            })
            .collect();

        self.guessing_word = result;

        println!("\t\tword to guess: {}", self.guessing_word);
        println!("\t\t{} lives remaining..", self.lives);
        // println!("{} letters to guess", self.letters_left_to_guess);
        // println!("{}", self.correctly_guessed_chars);
        println!("\t\t[{}] are not included", self.incorrectly_guessed_chars);
    }

    pub fn accept_guess(&mut self) {
        loop {
            self.guess.truncate(0);
            io::stdin()
                .read_line(&mut self.guess)
                .expect("failed to accept guess");

            self.guess = self.guess.trim().to_string();
            // println!("you attempted to guess: {}", self.guess);

            if self.guess.len() == 1 {
                break;
            } else {
                println!("1 letter at a time..");
            }
        }
    }

    pub fn check(&mut self) {
        loop {
            if self.incorrectly_guessed_chars.contains(&self.guess)
                || self.correctly_guessed_chars.contains(&self.guess)
            {
                self.update_game_state();
            } else if self.word.contains(&self.guess) {
                // first correct guess
                self.successful_guess();
                self.update_game_state();
            } else {
                // first failed guess
                self.fail_guess();
                self.update_game_state();
            }

            if self.lives == 0 {
                println!("\n\t\tyou died");
                break;
            }

            if self.letters_left_to_guess == 0 {
                println!("\n\t\tyou win! the word was: {}", self.word);
                break;
            }

            self.accept_guess();
        }
    }

    fn successful_guess(&mut self) {
        let mut count: usize = 0;
        // grab their current guess
        let character_to_check: char = self
            .guess
            .chars()
            .next()
            .expect("error when accessing guessed letter");
        // go through the whole word with their current guess
        for char in self.word.chars() {
            // if the guess they passed in matches a letter in the word, increase count
            if char == character_to_check {
                count += 1;
            }
        }
        // decrease the number of letters they have to guess by the count we got
        self.letters_left_to_guess -= count;
        // add the correctly guessed letter to our collection of correctly guessed letters
        self.correctly_guessed_chars.push_str(&self.guess);
        // let them know how many letters they have to guess
        // println!("letters left to guess: {}", self.letters_left_to_guess);
    }

    fn fail_guess(&mut self) {
        self.lives -= 1;
        self.incorrectly_guessed_chars.push_str(&self.guess);
        // println!("lives remaining: {}", self.lives);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn accepts_guess() {
        let mut game: Game = Game::new(String::from("tomato"));
        game.accept_guess();
        assert_ne!(0, game.guess.len());
    }
}

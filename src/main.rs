use hangman::Game;
fn main() {
    let game: Game = Game::new(String::from("potato"));

    // get player guess
    let mut guess: String = String::new();
    let guess_to_handle = Game::guess(&mut guess);

    // compare the guess to the word
    game.check(guess_to_handle);
}

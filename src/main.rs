use hangman::Game;
fn main() {
    let mut game: Game = Game::new(String::from("potato"));

    // game.intro();

    game.accept_guess();

    game.check();
}

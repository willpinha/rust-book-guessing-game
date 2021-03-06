use simple_guessing_game::{Game, User};

fn main() {
    let game = Game::new(1, 101);

    let mut guess = User::guess_number();

    while !game.validate_guess(&guess) {
        guess = User::guess_number();
    }
}

mod dice;
mod dice_result;
mod input;
mod game;
mod scores;

fn main() {
    game::Game::new().start();
}
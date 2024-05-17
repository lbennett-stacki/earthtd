use game::Game;

mod board;
mod coord;
mod game;
mod mob;

pub fn start() -> Game {
    let mut game = Game::default();

    game.start();

    println!("Game started! {:#?}", game);

    game
}

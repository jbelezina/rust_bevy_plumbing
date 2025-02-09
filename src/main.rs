mod game;
use game::Game;

use bevy::prelude::*;

fn main() {
    let game = Game::new();
    println!("Game board width: {}", game.board.width);
    println!("Game board height: {}", game.board.height);
    println!("Game board height: {:?}", game.board.tiles);

    App::new().run();
}
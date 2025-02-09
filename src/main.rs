mod game;
use game::{init_board, print_board_size};

use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, (init_board, print_board_size).chain())
        .run();
}

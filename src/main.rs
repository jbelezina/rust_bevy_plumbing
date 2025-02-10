mod game;
use game::{draw_hud, draw_tiles, init_board};

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init_board)
        .add_systems(Update, (draw_tiles, draw_hud))
        .run();
}

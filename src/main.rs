mod game;
use game::{draw_hud, handle_tile_selection, init_board, spawn_tiles, update_active_tile};

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(
            Startup,
            (init_board, spawn_tiles, draw_hud).chain(),
        )
        .add_systems(Update, (handle_tile_selection, update_active_tile).chain())
        .run();
}

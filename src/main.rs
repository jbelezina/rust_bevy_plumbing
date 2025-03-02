mod game;
use game::*;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, (init_board, spawn_tile_meshes, spawn_hud).chain())
        .add_systems(
            Update,
            (
                handle_tile_selection,
                handle_tile_shuffle,
                layout_tiles,
                update_active_tile,
            ),
        )
        .run();
}

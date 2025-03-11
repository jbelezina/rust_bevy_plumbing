mod game;
use game::*;

use bevy::prelude::*;
use bevy_2d_line::LineRenderingPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LineRenderingPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(
            Startup,
            (init_board, spawn_tiles, spawn_tile_meshes, spawn_hud).chain(),
        )
        .add_systems(
            Update,
            ((
                handle_tile_selection,
                handle_tile_shuffle,
                handle_tile_rotation,
                update_active_tile,
                layout_tiles,
            )
                .chain(),),
        )
        .run();
}

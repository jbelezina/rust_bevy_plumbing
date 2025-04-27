mod game;
use game::*;

use bevy::prelude::*;
use bevy_2d_line::LineRenderingPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LineRenderingPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<WaterTimer>()
        .add_systems(
            Startup,
            (init_board, spawn_tiles, spawn_tile_meshes, spawn_hud).chain(),
        )
        .add_systems(
            Update,
            ((
                tick_water_timer,
                handle_tile_select_shuffle,
                handle_tile_rotation,
                update_active_tile,
                update_display_next_water_idx,
                update_water,
                layout_tiles,
            )
                .chain(),),
        )
        .run();
}

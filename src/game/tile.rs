use bevy::prelude::*;
use bevy_2d_line::Line;

use super::{board::Board, pipe::Pipe};

#[derive(Component)]
#[require(Transform, Visibility)]
pub struct Tile {
    pub idx: i16,
    pub mesh_handle: Handle<Mesh>,
    pub material_handle: Handle<ColorMaterial>,
}

pub fn handle_tile_shuffle(
    keys: Res<ButtonInput<KeyCode>>,
    mut q_board: Query<&mut Board>,
    mut q_tiles: Query<(&mut Tile, &mut Visibility), With<Tile>>,
) {
    let mut board = q_board.single_mut();

    if keys.just_pressed(KeyCode::Enter) {
        let right = board.active_tile_idx + 1;
        let left = board.active_tile_idx - 1;
        let up = board.active_tile_idx + board.cols;
        let down = board.active_tile_idx - board.cols;

        if right == board.gap_idx
            || left == board.gap_idx
            || up == board.gap_idx
            || down == board.gap_idx
        {
            for (tile, mut visibility) in q_tiles.iter_mut() {
                if tile.idx == board.active_tile_idx {
                    *visibility = Visibility::Hidden;
                }
                if tile.idx == board.gap_idx {
                    *visibility = Visibility::Visible;
                }
            }
        }

        if right == board.gap_idx {
            board.gap_idx = board.active_tile_idx;
            board.active_tile_idx = right;
        }

        if left == board.gap_idx {
            board.gap_idx = board.active_tile_idx;
            board.active_tile_idx = left;
        }

        if up == board.gap_idx {
            board.gap_idx = board.active_tile_idx;
            board.active_tile_idx = up;
        }

        if down == board.gap_idx {
            board.gap_idx = board.active_tile_idx;
            board.active_tile_idx = down;
        }
    }
}

pub fn handle_tile_selection(keys: Res<ButtonInput<KeyCode>>, mut q_board: Query<&mut Board>) {
    let mut board = q_board.single_mut();

    if keys.just_pressed(KeyCode::ArrowRight) {
        let new_idx = board.active_tile_idx + 1;
        if new_idx <= board.size - 1 && new_idx != board.gap_idx {
            board.as_mut().active_tile_idx += 1;
        }
    }

    if keys.just_pressed(KeyCode::ArrowLeft) {
        let new_idx = board.active_tile_idx - 1;
        if new_idx >= 0 && new_idx != board.gap_idx {
            board.as_mut().active_tile_idx -= 1;
        }
    }

    if keys.just_pressed(KeyCode::ArrowDown) {
        let new_idx = board.active_tile_idx + board.cols;
        if new_idx <= board.size - 1 && new_idx != board.gap_idx {
            board.as_mut().active_tile_idx += board.cols;
        }
    }

    if keys.just_pressed(KeyCode::ArrowUp) {
        let new_idx = board.active_tile_idx - board.cols;
        if new_idx >= 0 && new_idx != board.gap_idx {
            board.as_mut().active_tile_idx -= board.cols;
        }
    }
}

pub fn handle_tile_rotation(
    keys: Res<ButtonInput<KeyCode>>,
    query_board: Query<&Board>,
    mut query: Query<(&Tile, Entity), With<Tile>>,
    mut q_lines: Query<(&mut Line, &mut Pipe, Entity), With<Pipe>>,
    children_query: Query<&Children>,
    mut commands: Commands,
) {
    let board = query_board.single();
    if keys.just_pressed(KeyCode::Space) {
        for (tile, tile_entity) in query.iter_mut() {
            if tile.idx == board.active_tile_idx {
                for pipe_child in children_query.iter_descendants(tile_entity) {
                    for (mut line, mut pipe, pipe_entity) in q_lines.iter_mut() {
                        if pipe_child == pipe_entity {
                            commands.entity(tile_entity).remove_children(&[pipe_entity]);
                            pipe.rotate();
                            line.points = pipe.pipe_type.get_points();
                            commands.entity(tile_entity).add_child(pipe_entity);
                        }
                    }
                }
            }
        }
    }
}

pub fn update_active_tile(
    q_board: Query<&Board>,
    tiles: Query<&Tile>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let board = q_board.single();

    for tile in tiles.iter() {
        if tile.idx == board.active_tile_idx {
            if let Some(material) = materials.get_mut(&tile.material_handle) {
                material.color = board.tile_color_active;
            }
        } else {
            if let Some(material) = materials.get_mut(&tile.material_handle) {
                if material.color != board.tile_color {
                    material.color = board.tile_color;
                }
            }
        }
    }
}

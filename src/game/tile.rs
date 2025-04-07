use bevy::prelude::*;
use bevy_2d_line::Line;

use super::{board::Board, pipe::Pipe};

#[derive(Component)]
#[require(Transform, Visibility)]
pub struct Tile {
    pub idx: i16,
    pub mesh_handle: Handle<Mesh>,
    pub material_handle: Handle<ColorMaterial>,
    pub movable: bool,
}

pub fn handle_tile_shuffle(
    keys: Res<ButtonInput<KeyCode>>,
    mut q_board: Query<&mut Board>,
    mut q_tiles: Query<(&mut Tile, &mut Visibility, Entity), With<Tile>>,
    children_query: Query<&Children>,
    mut commands: Commands,
) {
    let mut board = q_board.single_mut();

    for (tile, mut _visibility, _tile_entity) in q_tiles.iter() {
        if tile.idx == board.active_tile_idx && !tile.movable {
            return;
        }
    }

    let mut target_gap_index: Option<i16> = None; // idx of the tile the player is trying to shuffle the active tile towards

    if keys.just_pressed(KeyCode::ArrowRight) {
        target_gap_index = Some(board.active_tile_idx + 1);
    }

    if keys.just_pressed(KeyCode::ArrowLeft) {
        target_gap_index = Some(board.active_tile_idx - 1);
    }

    if keys.just_pressed(KeyCode::ArrowUp) {
        target_gap_index = Some(board.active_tile_idx - board.cols);
    }

    if keys.just_pressed(KeyCode::ArrowDown) {
        target_gap_index = Some(board.active_tile_idx + board.cols);
    }

    if target_gap_index.is_some() && board.gap_idxs.contains(&target_gap_index.unwrap()) {
        let mut pipe_to_append: Option<Entity> = None;
        let active_idx = board.active_tile_idx.clone();

        for (tile, mut visibility, tile_entity) in q_tiles.iter_mut() {
            if tile.idx == board.active_tile_idx {
                *visibility = Visibility::Hidden;
                for pipe_entity in children_query.iter_descendants(tile_entity) {
                    commands.entity(tile_entity).remove_children(&[pipe_entity]);
                    pipe_to_append = Some(pipe_entity);
                }
            }
            if board.gap_idxs.contains(&tile.idx) {
                *visibility = Visibility::Visible;
            }
        }

        if pipe_to_append.is_some() {
            let pipe = pipe_to_append.unwrap();
            for (tile, mut _visibility, tile_entity) in q_tiles.iter_mut() {
                if tile.idx == target_gap_index.unwrap() {
                    commands.entity(tile_entity).clear_children();
                    commands.entity(tile_entity).add_child(pipe);
                }
            }
        }

        board.gap_idxs.push(active_idx);
        board
            .gap_idxs
            .retain(|value| *value != target_gap_index.unwrap());
        board.active_tile_idx = target_gap_index.unwrap();
    }
}

pub fn handle_tile_selection(keys: Res<ButtonInput<KeyCode>>, mut q_board: Query<&mut Board>) {
    let mut board = q_board.single_mut();

    if keys.just_pressed(KeyCode::ArrowRight) {
        let new_idx = board.active_tile_idx + 1;

        if new_idx <= board.size - 1 && !board.gap_idxs.contains(&new_idx) {
            board.as_mut().active_tile_idx += 1;
        }
    }

    if keys.just_pressed(KeyCode::ArrowLeft) {
        let new_idx = board.active_tile_idx - 1;
        if new_idx >= 0 && !board.gap_idxs.contains(&new_idx) {
            board.as_mut().active_tile_idx -= 1;
        }
    }

    if keys.just_pressed(KeyCode::ArrowDown) {
        let new_idx = board.active_tile_idx + board.cols;
        if new_idx <= board.size - 1 && !board.gap_idxs.contains(&new_idx) {
            board.as_mut().active_tile_idx += board.cols;
        }
    }

    if keys.just_pressed(KeyCode::ArrowUp) {
        let new_idx = board.active_tile_idx - board.cols;
        if new_idx >= 0 && !board.gap_idxs.contains(&new_idx) {
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
            if tile.idx == board.active_tile_idx && tile.movable {
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

pub fn update_display_next_water_idx(
    q_board: Query<&Board>,
    tiles: Query<&Tile>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let board = q_board.single();

    for tile in tiles.iter() {
        if tile.idx == board.next_water_idx {
            if let Some(material) = materials.get_mut(&tile.material_handle) {
                material.color = Color::hsl(155.00, 0.95, 0.7);
            }
        }
    }
}

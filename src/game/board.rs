use bevy::prelude::*;
use bevy_2d_line::Line;
use rand::prelude::*;

use super::{
    pipe::{Pipe, PipeType, Point},
    tile::Tile,
};

#[derive(Component)]
pub struct Board {
    pub active_tile_idx: i16,
    pub water_idx: i16,
    pub next_water_idx: i16,
    pub next_entry_point: Point,
    pub size: i16,
    pub gap_idxs: Vec<i16>,
    pub cols: i16,
    pub rows: i16,
    pub tile_gap: f32,
    pub tile_size: f32,
    pub tile_color: Color,
    pub tile_color_active: Color,
}

const BOARD_ROWS: i16 = 10;
const BOARD_COLS: i16 = 14;

pub fn init_board(mut commands: Commands) {
    let board_size = BOARD_ROWS * BOARD_COLS;
    commands.spawn(Camera2d);
    commands.spawn((Board {
        active_tile_idx: 0,
        water_idx: 0,
        next_water_idx: BOARD_COLS,
        next_entry_point: Point::Top,
        size: board_size,
        gap_idxs: (|| {
            let mut idxs = Vec::new();
            let mut rng = rand::rng();
            for _i in 1..(BOARD_ROWS * 2) {
                let num = rng.random_range(1..(BOARD_COLS * BOARD_ROWS - 1));
                idxs.push(num);
            }
            return idxs;
        })(),
        rows: BOARD_ROWS,
        cols: BOARD_COLS,
        tile_gap: 5.0,
        tile_size: 50.0,
        tile_color: Color::hsl(255.00, 0.95, 0.7),
        tile_color_active: Color::linear_rgb(1.0, 1.0, 1.0),
    },));
}

pub fn spawn_tiles(
    mut commands: Commands,
    q_board: Query<&Board>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let board = q_board.single();
    let mut tile_idx = 0;

    for _ in 0..board.size {
        let tile = Tile {
            idx: tile_idx,
            mesh_handle: meshes.add(Rectangle::new(board.tile_size, board.tile_size)),
            material_handle: materials.add(if tile_idx == 0 {
                board.tile_color_active
            } else {
                board.tile_color
            }),
            movable: true,
        };

        commands.spawn(tile).with_children(|parent| {
            let colors = vec![LinearRgba::BLACK, LinearRgba::BLACK, LinearRgba::BLACK];
            let pipe_type = if tile_idx == 0 {
                PipeType::Straight(Point::Top)
            } else {
                PipeType::random()
            };

            parent.spawn((
                Pipe { pipe_type },
                Line {
                    points: pipe_type.get_points(),
                    colors,
                    thickness: 5.0,
                },
                Transform::from_xyz(0.0, 0.0, 0.1),
            ));
        });

        tile_idx += 1;
    }
}

pub fn spawn_tile_meshes(
    mut commands: Commands,
    query_tile: Query<(Entity, &Tile), With<Tile>>,
    query_board: Query<&Board>,
) {
    let board = query_board.single();

    for (entity_id, tile) in query_tile.iter() {
        commands
            .entity(entity_id)
            .insert(Mesh2d(tile.mesh_handle.clone()));

        commands
            .entity(entity_id)
            .insert(MeshMaterial2d(tile.material_handle.clone()));
        commands
            .entity(entity_id)
            .insert(if board.gap_idxs.contains(&tile.idx) {
                Visibility::Hidden
            } else {
                Visibility::Visible
            });
    }
}

pub fn spawn_hud(mut commands: Commands, board: Query<&Board>) {
    let b = board.single();

    commands.spawn(Text::new(format!("Active tile {}", b.active_tile_idx)));
}

pub fn layout_tiles(
    mut query: Query<(&mut Transform, &Tile, Entity), With<Tile>>,
    q_board: Query<&Board>,
    mut commands: Commands,
) {
    let board = q_board.single();

    let x_offset = (board.cols as f32 * (board.tile_size + board.tile_gap)) / 2.0 - board.tile_gap;
    let y_offset = (board.rows as f32 * (board.tile_size + board.tile_gap)) / 2.0 - board.tile_gap;

    for (mut transform, tile, tile_entity) in query.iter_mut() {
        let row = tile.idx / board.cols;
        let column = tile.idx % board.cols;

        if board.gap_idxs.contains(&tile.idx) {
            commands.entity(tile_entity).insert(Visibility::Hidden);
        }

        transform.translation.x = ((board.tile_size + board.tile_gap) * column as f32) - x_offset;
        transform.translation.y = -((board.tile_size + board.tile_gap) * row as f32) + y_offset;
        transform.translation.z = 0.0;
    }
}

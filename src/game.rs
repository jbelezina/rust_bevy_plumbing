use bevy::prelude::{Color, *};

const TILE_GAP: f32 = 10.0;
const TILE_SIZE: f32 = 50.0;
const TILE_COLOR: Color = Color::hsl(255.00, 0.95, 0.7);
const TILE_COLOR_ACTIVE: Color = Color::linear_rgb(1.0, 1.0, 1.0);
const BOARD_ROWS: i16 = 6;
const BOARD_COLS: i16 = 8;

#[derive(Component)]
pub struct Tile {
    idx: i16,
    is_active: bool,
    is_movable: bool,
    entry_points: Vec<i16>,
    rotation: i32,
    row: i32,
    col: i32,
    mesh_handle: Handle<Mesh>,
    material_handle: Handle<ColorMaterial>,
}

#[derive(Component)]
pub struct Board {
    active_tile_idx: i16,
    size: i16,
}

pub fn init_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    commands.spawn((
        Board { active_tile_idx: 0, size: BOARD_ROWS * BOARD_COLS },
    ));

    let mut tile_idx = 0;

    for r in 0..BOARD_ROWS {
        for c in 0..BOARD_COLS {
            let tile = Tile {
                idx: tile_idx,
                is_active: if c == 0 && r == 0 { true } else { false },
                is_movable: false,
                row: r as i32,
                col: c as i32,
                entry_points: vec![4, 6],
                rotation: 0,
                mesh_handle: meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE)),
                material_handle: materials.add(if tile_idx == 0 {
                    TILE_COLOR_ACTIVE
                } else {
                    TILE_COLOR
                }),
            };

            commands.spawn((tile,));
            tile_idx += 1;
        }
    }
}

pub fn spawn_tiles(mut commands: Commands, tiles: Query<&Tile>) {
    let x_offset = (BOARD_COLS as f32 * (TILE_SIZE + TILE_GAP)) / 2.0 - TILE_GAP;
    let y_offset = (BOARD_ROWS as f32 * (TILE_SIZE + TILE_GAP)) / 2.0 - TILE_GAP;

    for tile in tiles.iter() {
        commands.spawn((
            Mesh2d(tile.mesh_handle.clone()),
            MeshMaterial2d(tile.material_handle.clone()),
            Transform::from_xyz(
                ((TILE_SIZE + TILE_GAP) * tile.col as f32) - x_offset,
                -((TILE_SIZE + TILE_GAP) * tile.row as f32) + y_offset,
                0.0,
            ),
        ));
    }
}

pub fn draw_hud(mut commands: Commands, board: Query<&Board>) {
    let b = board.single();

    commands.spawn(Text::new(format!("Active tile {}", b.active_tile_idx)));
}

pub fn handle_tile_selection(keys: Res<ButtonInput<KeyCode>>, mut board: Query<&mut Board>) {
    let mut my_board = board.single_mut();
    if keys.just_pressed(KeyCode::ArrowRight) {
        if my_board.active_tile_idx + 1 <= my_board.size - 1 {
            my_board.as_mut().active_tile_idx += 1;
        }
    }

    if keys.just_pressed(KeyCode::ArrowLeft) {
        if my_board.active_tile_idx - 1 >= 0 {
        my_board.as_mut().active_tile_idx -= 1;
        }
    }

    if keys.just_pressed(KeyCode::ArrowDown) {
        if my_board.active_tile_idx + BOARD_COLS <= my_board.size - 1 {
            my_board.as_mut().active_tile_idx += BOARD_COLS;
        }
    }

    if keys.just_pressed(KeyCode::ArrowUp) {
        if my_board.active_tile_idx - BOARD_COLS >= 0 {
            my_board.as_mut().active_tile_idx -= BOARD_COLS;
        }
    }
}

pub fn update_active_tile(
    board: Query<&Board>,
    tiles: Query<&Tile>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let my_board = board.single();

    for tile in tiles.iter() {
        if (tile.idx == my_board.active_tile_idx) {
            println!("active tile {}", my_board.active_tile_idx);
            if let Some(material) = materials.get_mut(&tile.material_handle) {
                material.color = TILE_COLOR_ACTIVE;
            }
        } else {
            if let Some(material) = materials.get_mut(&tile.material_handle) {
                if(material.color != TILE_COLOR) {
                    material.color = TILE_COLOR;
                }
            }
        }
    }
}

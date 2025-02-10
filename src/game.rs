use bevy::prelude::{*, Color};

const TILE_GAP: f32 = 10.0;
const TILE_SIZE: f32 = 50.0;
const TILE_COLOR: Color = Color::hsl(255.00, 0.95, 0.7);
const TILE_COLOR_ACTIVE: Color = Color::linear_rgb(1.0, 1.0, 1.0);
const BOARD_ROWS: i16 = 6;
const BOARD_COLS: i16 = 8;

#[derive(Component)]
pub struct Tile {
    is_active: bool,
    is_movable: bool,
    entry_points: Vec<i16>,
    rotation: i32,
    row: i32,
    col: i32,
}

#[derive(Component)]
pub struct Dimensions {
    rows: i16,
    cols: i16,
}

#[derive(Component)]
pub struct Board;

pub fn init_board(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Board,
        Dimensions {
            rows: BOARD_ROWS,
            cols: BOARD_COLS,
        },
    ));

    for r in 0..BOARD_ROWS {
        for c in 0..BOARD_COLS {
            commands.spawn(Tile {
                is_active: if c == 0 && r == 0 { true } else { false },
                is_movable: false,
                row: r as i32,
                col: c as i32,
                entry_points: vec![4, 6],
                rotation: 0,
            });
        }
    }
}

pub fn draw_hud(
    mut commands: Commands,
    board_dimensions: Query<&Dimensions, With<Board>>,
) {
    let dimensions = board_dimensions.single();

    commands.spawn(Text::new(format!(
        "Tiles horizonally: {} vertically: {}",
        dimensions.cols, dimensions.rows
    )));
}

pub fn draw_tiles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    tiles: Query<&Tile>,
    board_dimensions: Query<&Dimensions, With<Board>>,
) {

    let dimensions = board_dimensions.single();
    let x_offset = (dimensions.cols as f32 * (TILE_SIZE + TILE_GAP)) / 2.0 - TILE_GAP;
    let y_offset = (dimensions.rows as f32 * (TILE_SIZE + TILE_GAP)) / 2.0 - TILE_GAP;

    for tile in &tiles {
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE))),
            MeshMaterial2d(materials.add(if tile.is_active { TILE_COLOR_ACTIVE } else { TILE_COLOR })),
            Transform::from_xyz(
                ((TILE_SIZE + TILE_GAP) * tile.col as f32) - x_offset,
                -((TILE_SIZE + TILE_GAP) * tile.row as f32) + y_offset,
                0.0,
            ),
        ));
    }
}

use bevy::prelude::{Color, *};

const TILE_GAP: f32 = 10.0;
const TILE_SIZE: f32 = 50.0;
const TILE_COLOR: Color = Color::hsl(255.00, 0.95, 0.7);
const TILE_COLOR_ACTIVE: Color = Color::linear_rgb(1.0, 1.0, 1.0);
const BOARD_ROWS: i16 = 10;
const BOARD_COLS: i16 = 14;

#[derive(Component)]
#[require(Transform, Visibility)]
pub struct Tile {
    idx: i16,
    entry_points: Vec<i16>,
    rotation: i32,
    mesh_handle: Handle<Mesh>,
    material_handle: Handle<ColorMaterial>,
}

#[derive(Component)]
pub struct Board {
    active_tile_idx: i16,
    size: i16,
    gap_idx: i16,
}

pub fn init_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let gap_idx = (BOARD_ROWS * BOARD_COLS) - 1;
    let board_size = BOARD_ROWS * BOARD_COLS;
    commands.spawn(Camera2d);
    commands.spawn((Board {
        active_tile_idx: 0,
        size: board_size,
        gap_idx,
    },));

    let mut tile_idx = 0;

    for _ in 0..board_size {
        let tile = Tile {
            idx: tile_idx,
            entry_points: vec![4, 6],
            rotation: 0,
            mesh_handle: meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE)),
            material_handle: materials.add(if tile_idx == 0 {
                TILE_COLOR_ACTIVE
            } else {
                TILE_COLOR
            }),
        };

        commands.spawn(tile);

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
            .insert(if board.gap_idx == tile.idx {
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
        let new_idx = board.active_tile_idx + BOARD_COLS;
        if new_idx <= board.size - 1 && new_idx != board.gap_idx {
            board.as_mut().active_tile_idx += BOARD_COLS;
        }
    }

    if keys.just_pressed(KeyCode::ArrowUp) {
        let new_idx = board.active_tile_idx - BOARD_COLS;
        if new_idx >= 0 && new_idx != board.gap_idx {
            board.as_mut().active_tile_idx -= BOARD_COLS;
        }
    }
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
        let up = board.active_tile_idx + BOARD_COLS;
        let down = board.active_tile_idx - BOARD_COLS;

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

pub fn update_active_tile(
    board: Query<&Board>,
    tiles: Query<&Tile>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let my_board = board.single();

    for tile in tiles.iter() {
        if tile.idx == my_board.active_tile_idx {
            if let Some(material) = materials.get_mut(&tile.material_handle) {
                material.color = TILE_COLOR_ACTIVE;
            }
        } else {
            if let Some(material) = materials.get_mut(&tile.material_handle) {
                if material.color != TILE_COLOR {
                    material.color = TILE_COLOR;
                }
            }
        }
    }
}

pub fn layout_tiles(mut query: Query<(&mut Transform, &Tile), With<Tile>>) {
    let x_offset = (BOARD_COLS as f32 * (TILE_SIZE + TILE_GAP)) / 2.0 - TILE_GAP;
    let y_offset = (BOARD_ROWS as f32 * (TILE_SIZE + TILE_GAP)) / 2.0 - TILE_GAP;

    for (mut transform, tile) in query.iter_mut() {
        let row = tile.idx / BOARD_COLS;
        let column = tile.idx % BOARD_COLS;

        transform.translation.x = ((TILE_SIZE + TILE_GAP) * column as f32) - x_offset;
        transform.translation.y = -((TILE_SIZE + TILE_GAP) * row as f32) + y_offset;
        transform.translation.z = 0.0;
    }
}

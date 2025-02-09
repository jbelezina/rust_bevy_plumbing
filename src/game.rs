use bevy::prelude::*;

#[derive(Component)]
pub struct Dimensions {
    width: i16,
    height: i16,
}

#[derive(Component)]
pub struct Board;

pub fn init_board(mut commands: Commands) {
    commands.spawn((
        Board,
        Dimensions {
            width: 4,
            height: 4,
        },
    ));
}

pub fn print_board_size(query: Query<&Dimensions, With<Board>>) {
    for dimensions in &query {
        println!("width: {}, height: {}", dimensions.width, dimensions.height);
    }
}

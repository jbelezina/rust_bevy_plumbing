use bevy::prelude::*;
use bevy_2d_line::Line;

use super::{
    pipe::{Pipe, Point},
    Board, Tile,
};

#[derive(Resource)]
pub struct WaterTimer(Timer);

impl WaterTimer {
    pub fn new() -> Self {
        Self(Timer::from_seconds(2.0, TimerMode::Repeating))
    }
}

impl Default for WaterTimer {
    fn default() -> Self {
        Self::new()
    }
}

pub fn tick_water_timer(time: Res<Time>, mut water_timer: ResMut<WaterTimer>) {
    water_timer.0.tick(time.delta());
}

pub fn update_water(
    mut q_tiles: Query<(&mut Tile, Entity), With<Tile>>,
    children_query: Query<&Children>,
    mut q_lines: Query<(&mut Line, &Pipe, Entity), With<Pipe>>,
    mut q_board: Query<&mut Board>,
    water_timer: Res<WaterTimer>,
) {
    let mut board = q_board.single_mut();
    for (mut tile, tile_entity) in q_tiles.iter_mut() {
        if tile.idx == board.water_idx {
            let mut ready_to_paint = false;

            for _pipe_entity in children_query.iter_descendants(tile_entity) {
                for (mut line, _pipe, _line_pipe_entity) in q_lines.iter_mut() {
                    if *line.points.first().unwrap() == board.next_entry_point.get() {
                        ready_to_paint = true;
                    } else if *line.points.last().unwrap() == board.next_entry_point.get() {
                        line.points.reverse();
                        ready_to_paint = true;
                    }
                }
            }

            if ready_to_paint && water_timer.0.just_finished() {
                for pipe_entity in children_query.iter_descendants(tile_entity) {
                    for (mut line, _pipe, line_pipe_entity) in q_lines.iter_mut() {
                        if line_pipe_entity == pipe_entity {
                            let idx_to_color =
                                line.colors.iter().position(|c| *c == LinearRgba::BLACK);

                            if idx_to_color.is_some() {
                                line.colors[idx_to_color.unwrap()] = LinearRgba::BLUE;
                                tile.movable = false;
                            } else {
                                let exit_point: Vec2 = *line.points.last().unwrap();

                                match exit_point {
                                    exit_point if exit_point == Point::Top.get() => {
                                        board.next_entry_point = Point::Top.get_oposite();
                                        board.next_water_idx = tile.idx - board.cols;
                                    }
                                    exit_point if exit_point == Point::Right.get() => {
                                        board.next_entry_point = Point::Right.get_oposite();
                                        board.next_water_idx = tile.idx + 1;
                                    }

                                    exit_point if exit_point == Point::Bottom.get() => {
                                        board.next_entry_point = Point::Bottom.get_oposite();
                                        board.next_water_idx = tile.idx + board.cols;
                                    }

                                    exit_point if exit_point == Point::Left.get() => {
                                        board.next_entry_point = Point::Left.get_oposite();
                                        board.next_water_idx = tile.idx - 1;
                                    }

                                    _ => (),
                                }
                                board.water_idx = board.next_water_idx;
                            }
                        }
                    }
                }
            }
        }
    }
}

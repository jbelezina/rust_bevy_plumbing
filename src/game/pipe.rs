use bevy::prelude::*;
use rand::Rng;

#[derive(Clone, Copy)]
pub enum PipeOrientation {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Clone, Copy)]
pub enum PipeType {
    Elbow(PipeOrientation),
    // Cross(PipeOrientation),
    // T(PipeOrientation),
    Straight(PipeOrientation),
}

impl PipeType {
    pub fn get_points(&self) -> Vec<Vec2> {
        match *self {
            PipeType::Straight(PipeOrientation::Top) => vec![
                Vec2::new(-25.0, 0.0),
                Vec2::new(0.0, 0.0),
                Vec2::new(25.0, 0.0),
            ],

            PipeType::Straight(PipeOrientation::Bottom) => vec![
                Vec2::new(-25.0, 0.0),
                Vec2::new(0.0, 0.0),
                Vec2::new(25.0, 0.0),
            ],

            PipeType::Straight(PipeOrientation::Left) => vec![
                Vec2::new(0.0, -25.0),
                Vec2::new(0.0, 0.0),
                Vec2::new(0.0, 25.0),
            ],

            PipeType::Straight(PipeOrientation::Right) => vec![
                Vec2::new(0.0, -25.0),
                Vec2::new(0.0, 0.0),
                Vec2::new(0.0, 25.0),
            ],

            PipeType::Elbow(PipeOrientation::Top) => vec![
                Vec2::new(0.0, 25.0),
                Vec2::new(0.0, 0.0),
                Vec2::new(25.0, 0.0),
            ],

            PipeType::Elbow(PipeOrientation::Right) => vec![
                Vec2::new(25.0, 0.0),
                Vec2::new(0.0, 0.0),
                Vec2::new(0.0, -25.0),
            ],

            PipeType::Elbow(PipeOrientation::Bottom) => vec![
                Vec2::new(0.0, -25.0),
                Vec2::new(0.0, 0.0),
                Vec2::new(-25.0, 0.0),
            ],

            PipeType::Elbow(PipeOrientation::Left) => vec![
                Vec2::new(-25.0, 0.0),
                Vec2::new(0.0, 0.0),
                Vec2::new(0.0, 25.0),
            ],
        }
    }

    pub fn random() -> PipeType {
        let variants = vec![
            PipeType::Straight(PipeOrientation::Top),
            PipeType::Straight(PipeOrientation::Right),
            PipeType::Straight(PipeOrientation::Bottom),
            PipeType::Straight(PipeOrientation::Left),
            PipeType::Elbow(PipeOrientation::Top),
            PipeType::Elbow(PipeOrientation::Right),
            PipeType::Elbow(PipeOrientation::Bottom),
            PipeType::Elbow(PipeOrientation::Left),
        ];

        let mut range = rand::rng();
        let random_idx = range.random_range(0..(variants.len() - 1));
        *variants.get(random_idx).unwrap()
    }
}

#[derive(Component)]
pub struct Pipe {
    pub pipe_type: PipeType,
}

impl Pipe {
    pub fn rotate(&mut self) {
        let next_pipe_type = match self.pipe_type {
            PipeType::Straight(PipeOrientation::Top) => PipeType::Straight(PipeOrientation::Right),
            PipeType::Straight(PipeOrientation::Right) => {
                PipeType::Straight(PipeOrientation::Bottom)
            }
            PipeType::Straight(PipeOrientation::Bottom) => {
                PipeType::Straight(PipeOrientation::Left)
            }
            PipeType::Straight(PipeOrientation::Left) => PipeType::Straight(PipeOrientation::Top),
            PipeType::Elbow(PipeOrientation::Top) => PipeType::Elbow(PipeOrientation::Right),
            PipeType::Elbow(PipeOrientation::Right) => PipeType::Elbow(PipeOrientation::Bottom),
            PipeType::Elbow(PipeOrientation::Bottom) => PipeType::Elbow(PipeOrientation::Left),
            PipeType::Elbow(PipeOrientation::Left) => PipeType::Elbow(PipeOrientation::Top),
        };

        self.pipe_type = next_pipe_type;
    }
}

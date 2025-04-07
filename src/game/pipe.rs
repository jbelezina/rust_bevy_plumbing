use bevy::prelude::*;
use rand::Rng;

#[derive(Clone, Copy)]
pub enum Point {
    Top,
    Right,
    Bottom,
    Left,
}

impl Point {
    pub fn get(&self) -> Vec2 {
        match self {
            Point::Top => Vec2::new(0.0, 25.0),
            Point::Bottom => Vec2::new(0.0, -25.0),
            Point::Left => Vec2::new(-25.0, 0.0),
            Point::Right => Vec2::new(25.0, 0.0),
        }
    }

    pub fn get_oposite(&self) -> Point {
        match self {
            Point::Top => Point::Bottom,
            Point::Right => Point::Left,
            Point::Bottom => Point::Top,
            Point::Left => Point::Right,
        }
    }
}

#[derive(Clone, Copy)]
pub enum PipeType {
    Elbow(Point),
    Straight(Point),
}

impl PipeType {
    pub fn get_points(&self) -> Vec<Vec2> {
        match *self {
            PipeType::Straight(Point::Top) => {
                vec![Point::Top.get(), Vec2::new(0.0, 0.0), Point::Bottom.get()]
            }

            PipeType::Straight(Point::Right) => {
                vec![Point::Right.get(), Vec2::new(0.0, 0.0), Point::Left.get()]
            }

            PipeType::Straight(Point::Bottom) => {
                vec![Point::Bottom.get(), Vec2::new(0.0, 0.0), Point::Top.get()]
            }

            PipeType::Straight(Point::Left) => {
                vec![Point::Left.get(), Vec2::new(0.0, 0.0), Point::Right.get()]
            }

            PipeType::Elbow(Point::Top) => {
                vec![Point::Top.get(), Vec2::new(0.0, 0.0), Point::Right.get()]
            }

            PipeType::Elbow(Point::Right) => {
                vec![Point::Right.get(), Vec2::new(0.0, 0.0), Point::Bottom.get()]
            }

            PipeType::Elbow(Point::Bottom) => {
                vec![Point::Bottom.get(), Vec2::new(0.0, 0.0), Point::Left.get()]
            }

            PipeType::Elbow(Point::Left) => {
                vec![Point::Left.get(), Vec2::new(0.0, 0.0), Point::Top.get()]
            }
        }
    }

    pub fn random() -> PipeType {
        let variants = vec![
            PipeType::Straight(Point::Top),
            PipeType::Straight(Point::Right),
            PipeType::Straight(Point::Bottom),
            PipeType::Straight(Point::Left),
            PipeType::Elbow(Point::Top),
            PipeType::Elbow(Point::Right),
            PipeType::Elbow(Point::Bottom),
            PipeType::Elbow(Point::Left),
        ];

        let mut range = rand::rng();
        let random_idx = range.random_range(0..(variants.len() - 1));
        *variants.get(random_idx).unwrap()
    }

    fn rotate(&self) -> PipeType {
        match self {
            PipeType::Straight(Point::Top) => PipeType::Straight(Point::Right),
            PipeType::Straight(Point::Right) => PipeType::Straight(Point::Bottom),
            PipeType::Straight(Point::Bottom) => PipeType::Straight(Point::Left),
            PipeType::Straight(Point::Left) => PipeType::Straight(Point::Top),
            PipeType::Elbow(Point::Top) => PipeType::Elbow(Point::Right),
            PipeType::Elbow(Point::Right) => PipeType::Elbow(Point::Bottom),
            PipeType::Elbow(Point::Bottom) => PipeType::Elbow(Point::Left),
            PipeType::Elbow(Point::Left) => PipeType::Elbow(Point::Top),
        }
    }
}

#[derive(Component)]
pub struct Pipe {
    pub pipe_type: PipeType,
}

impl Pipe {
    pub fn rotate(&mut self) {
        self.pipe_type = self.pipe_type.rotate();
    }
}

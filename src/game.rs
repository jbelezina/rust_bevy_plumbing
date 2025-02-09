
pub struct Board {
    pub width: i16,
    pub height: i16,
    pub tiles: Vec<i16>
}

impl Board {
    fn new() -> Self {
        Self {
            width: 3,
            height: 3,
            tiles: vec![1,1,1,1,1,1,1,1,0]
        }
    }
}
pub struct Game {
    pub board: Board,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
        }
    }
}
#[derive(Debug, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

impl Position {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y }
    }
}

pub struct Board<const N: usize> {
    pub(crate) array: [[i32; N]; N]
}

impl<const N: usize> Board<N> {
    const NEIGHBOURS: [Position; 4] = [
        Position { x: 0, y: 1 }, Position { x: 0, y: -1 }, 
        Position { x: 1, y: 0 }, Position { x: -1, y: 0 }
    ];

    pub const fn new() -> Self {
        Self { array: [[0; N]; N] }
    }

    pub fn neighbours(&self, position: &Position) -> Vec<Position> {
        Self::NEIGHBOURS
            .iter()
            .map(|p| { position.add(p) })
            .filter(Self::in_bounds)
            .collect()
    }

    fn in_bounds(&Position {x, y}: &Position) -> bool {
        x >= 0 && x < N as i32 && y >= 0 && y < N as i32
    }
}

impl<const N: usize> Default for Board<N> {
    fn default() -> Self {
        Self::new()
    }
}
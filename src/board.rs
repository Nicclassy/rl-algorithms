use std::ops::{Add, Index, IndexMut};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

impl Position {
    const UP: Self = Self { x: 0, y: -1 };
    const DOWN: Self = Self { x: 0, y: 1 };
    const LEFT: Self = Self { x: 1, y: 0 };
    const RIGHT: Self = Self { x: -1, y: 0 };

    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn in_direction(self, direction: Direction) -> Self {
        match direction {
            Direction::Up => self.add(Self::UP),
            Direction::Down => self.add(Self::DOWN),
            Direction::Left => self.add(Self::LEFT),
            Direction::Right => self.add(Self::RIGHT)
        }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

pub struct Board<const N: usize, T = i32> {
    array: [[T; N]; N]
}

impl<const N: usize, T: Default + Copy> Board<N, T> {
    const NEIGHBOURS: [Position; 4] = [
        Position::UP, Position::DOWN, 
        Position::LEFT, Position::RIGHT 
    ];

    pub fn new() -> Self {
        Self { array: [[Default::default(); N]; N] }
    }

    pub fn neighbours(&self, position: Position) -> Vec<Position> {
        Self::NEIGHBOURS
            .iter()
            .map(|&p| { position.add(p) })
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

impl<const N: usize, T> Index<&Position> for Board<N, T> {
    type Output = T;

    fn index(&self, index: &Position) -> &Self::Output {
        &self.array[index.y as usize][index.x as usize]
    }
}

impl<const N: usize, T> IndexMut<&Position> for Board<N, T> {
    fn index_mut(&mut self, index: &Position) -> &mut Self::Output {
        &mut self.array[index.y as usize][index.x as usize]
    }
}
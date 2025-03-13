use std::fmt::{self, Display};
use std::collections::HashMap;
use std::ops::{Add, Index, IndexMut};

use strum_macros::EnumIter;

use crate::env::Tile;
use crate::states::Action;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

#[derive(EnumIter, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn to_action(&self) -> Action {
        match self {
            Self::Up => 0,
            Self::Down => 1,
            Self::Left => 2,
            Self::Right => 3
        }
    }
}

impl Position {
    const UP: Self = Self { x: 0, y: -1 };
    const DOWN: Self = Self { x: 0, y: 1 };
    const LEFT: Self = Self { x: 1, y: 0 };
    const RIGHT: Self = Self { x: -1, y: 0 };

    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn in_direction(self, direction: Direction) -> Self {
        match direction {
            Direction::Up => self + Self::UP,
            Direction::Down => self + Self::DOWN,
            Direction::Left => self + Self::LEFT,
            Direction::Right => self + Self::RIGHT
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(x={}, y={})", self.x, self.y)
    } 
} 

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

#[derive(Clone)]
pub struct Board {
    tile_overrides: HashMap<Position, Tile>,
    array: Vec<Vec<Tile>>,
    pub size: usize
}

impl Board {
    pub fn new(tile_overrides: HashMap<Position, Tile>, size: usize) -> Self {
        let mut array = vec![vec![Tile::default(); size]; size];
        for (position, tile) in &tile_overrides {
            assert!(
                Self::in_bounds(position, size), 
                "positions must be within the board dimensions"
            );
            array[position.y as usize][position.x as usize] = *tile;
        }
        Self { tile_overrides, array, size }
    }

    pub fn reset(&mut self) {
        self.array = vec![vec![Tile::default(); self.size]; self.size];
        for (position, tile) in &self.tile_overrides {
            self.array[position.y as usize][position.x as usize] = *tile;
        }
    }

    pub(crate) fn all_positions(&self) -> Vec<Position> {
        (0..self.size)
            .flat_map(move |y| (0..self.size).map(move |x| Position::new(x as i32, y as i32)))
            .collect()
    }
 
    pub(crate) fn in_bounds(&Position {x, y}: &Position, size: usize) -> bool {
        x >= 0 && x < size as i32 && y >= 0 && y < size as i32
    }
}

impl Index<&Position> for Board {
    type Output = Tile;

    fn index(&self, index: &Position) -> &Self::Output {
        &self.array[index.y as usize][index.x as usize]
    }
}

impl IndexMut<&Position> for Board {
    fn index_mut(&mut self, index: &Position) -> &mut Self::Output {
        &mut self.array[index.y as usize][index.x as usize]
    }
}
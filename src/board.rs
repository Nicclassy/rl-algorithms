use std::collections::HashMap;
use std::ops::{Add, Index, IndexMut};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::env::Tile;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

#[derive(EnumIter, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
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
            Direction::Up => self + Self::UP,
            Direction::Down => self + Self::DOWN,
            Direction::Left => self + Self::LEFT,
            Direction::Right => self + Self::RIGHT
        }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

pub struct Board<const N: usize, T = Tile> {
    special_tiles: HashMap<Position, T>,
    array: [[T; N]; N]
}

impl<const N: usize, T: Default + Copy> Board<N, T> {
    pub fn new(special_tiles: HashMap<Position, T>) -> Self {
        let mut array = [[T::default(); N]; N];
        for (position, tile) in &special_tiles {
            array[position.y as usize][position.x as usize] = *tile;
        }
        Self { special_tiles, array }
    }

    pub fn reset(&mut self) {
        self.array = [[T::default(); N]; N];
        for (position, tile) in &self.special_tiles {
            self.array[position.y as usize][position.x as usize] = *tile;
        }
    }

    pub fn neighbours(&self, position: Position) -> (Vec<Direction>, Vec<Position>) {
        Direction::iter()
            .filter_map(|dir| {
                let neighbour = position.in_direction(dir);
                Self::in_bounds(&neighbour).then_some((dir, neighbour))
            })
            .unzip()
    }

    fn in_bounds(&Position {x, y}: &Position) -> bool {
        x >= 0 && x < N as i32 && y >= 0 && y < N as i32
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
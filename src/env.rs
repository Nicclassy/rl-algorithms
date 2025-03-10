use std::collections::HashMap;

use crate::{board::{Board, Position}, states::{Action, Actions}};

pub(crate) type Reward = i32;

#[derive(Eq, PartialEq, Hash)]
pub enum Tile {
    Curse,
    Gem,
    Goal
}

impl Tile {
    pub(crate) fn default_reward(&self) -> Reward {
        match self {
            Self::Curse => -10,
            Self::Gem => 5,
            Self::Goal => 20
        }
    }
}

pub struct TileRewards {
    pub(crate) state_rewards: HashMap<Tile, Reward>
}

impl TileRewards {
    pub fn new(state_rewards: HashMap<Tile, Reward>) -> Self {
        Self { state_rewards }
    }

    pub fn reward(&self, tile: &Tile) -> Reward {
        *self.state_rewards.get(tile).unwrap_or(&tile.default_reward())
    }
}

pub struct Agent {
    initial_position: Position,
    position: Position
}

impl Agent {
    pub fn new(initial_position: Position) -> Self {
        Self { initial_position, position: initial_position }
    }

    pub fn reset(&mut self) {
        self.position = self.initial_position
    }
}

pub struct Env<'a, const N: usize, T = i32> {
    agent: &'a Agent,
    actions: Actions,
    board: Board<N, T>,
}

impl<'a, const N: usize, T: Default + Copy> Env<'a, N, T> {
    pub fn new(agent: &'a Agent, actions: Actions) -> Self {
        Self { agent, actions, board: Board::new() }
    }

    pub fn reset(&mut self) {}
    pub fn step(&mut self, action: Action) {}
}
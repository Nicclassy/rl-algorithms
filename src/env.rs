use std::collections::HashMap;

use crate::{board::{Board, Position}, states::{Action, Actions}};

pub(crate) type Reward = i32;
pub(crate) type Rewarder = fn(Action, Agent) -> Reward;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum Tile {
    Normal,
    Curse,
    Gem,
    Goal
}

impl Tile {
    pub(crate) fn default_reward(&self) -> Reward {
        match self {
            Self::Normal => 0,
            Self::Curse => -10,
            Self::Gem => 5,
            Self::Goal => 20
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self::Normal
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
    pub position: Position,
    pub has_reached_goal: bool
}

impl Agent {
    pub fn new(initial_position: Position) -> Self {
        Self { initial_position, position: initial_position, has_reached_goal: false }
    }

    pub fn reset(&mut self) {
        self.position = self.initial_position
    }
}

pub struct Env<'a, const N: usize, T = i32> {
    board: Board<N, T>,
    agent: &'a mut Agent,
    actions: Actions,
    rewarder: Rewarder
}

impl<'a, const N: usize, T: Default + Copy> Env<'a, N, T> {
    pub fn new(agent: &'a mut Agent, actions: Actions, rewarder: Rewarder) -> Self {
        Self { board: Board::new(), agent, actions, rewarder }
    }

    pub fn reset(&mut self) {
        self.agent.reset();
        self.board.reset();
    }

    pub fn step(&mut self, action: Action) {

    }
}
use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::board::{Board, Direction, Position};
use crate::states::{Action, AgentAction};

pub(crate) type Reward = f32;
pub(crate) type Rewarder = fn(&Env) -> Reward;

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
            Self::Normal => 0f32,
            Self::Curse => -10f32,
            Self::Gem => 5f32,
            Self::Goal => 20f32
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
    pub path: Vec<Position>,
    pub has_reached_goal: bool
}

impl Agent {
    pub fn new(initial_position: Position) -> Self {
        Self { initial_position, position: initial_position, path: Vec::new(), has_reached_goal: false }
    }

    pub fn reset(&mut self) {
        self.position = self.initial_position;
        self.has_reached_goal = false;
        self.path.clear();
    }
}

pub struct Env<'a> {
    board: &'a mut Board,
    agent: &'a mut Agent,
    rewarder: Rewarder
}

impl<'a> Env<'a> {
    pub fn new(board: &'a mut Board, agent: &'a mut Agent, rewarder: Rewarder) -> Self {
        Self { board, agent, rewarder }
    }

    pub fn reset(&mut self) {
        self.agent.reset();
        self.board.reset();
    }

    pub fn step(&mut self, action: AgentAction) -> Reward {
        self.agent.position = action;
        self.agent.path.push(action);

        let reward = (self.rewarder)(self);
        let tile = &mut self.board[&action];
        match *tile {
            Tile::Goal => self.agent.has_reached_goal = true,
            Tile::Curse | Tile::Gem => *tile = Tile::Normal,
            _ => {}
        }

        reward
    }

    pub(crate) fn board(&self) -> Board {
        self.board.clone()
    }
 
    pub(crate) fn available_agent_positions(&self) -> HashMap<Action, Position> {
        Direction::iter()
            .filter_map(|dir| {
                let neighbour = self.agent.position.in_direction(dir);
                Board::in_bounds(&neighbour, self.board.size).then_some((dir.to_action(), neighbour))
            })
            .collect()
    }

    pub(crate) fn agent_has_reached_goal(&self) -> bool {
        self.agent.has_reached_goal
    }

    pub(crate) fn agent_position(&self) -> Position {
        self.agent.position
    }

    pub(crate) fn agent_path(&self) -> Vec<Position> {
        self.agent.path.clone()
    }

    pub(crate) fn agent_tile(&self) -> Tile {
        self.board[&self.agent.position]
    }
}
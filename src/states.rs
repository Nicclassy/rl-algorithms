use std::{collections::HashMap, ops::Index};

use rand::seq::IndexedRandom;

use crate::board::Position;

pub(crate) type State = usize;
pub(crate) type Action = usize;

pub struct States {
    mapping: HashMap<Position, State>
}

impl States {
    pub fn new(n_possible_states: usize, positions: &[Position]) -> Self {
        let mut mapping = HashMap::with_capacity(n_possible_states);
        for (state, position) in positions.iter().enumerate() {
            mapping.insert(*position, state as State);
        }
        Self { mapping }
    }
}

impl Index<&Position> for States {
    type Output = State;

    fn index(&self, position: &Position) -> &State {
        &self.mapping[position]
    }
}

pub struct Actions {
    values: Vec<Action>
}

impl Actions {
    pub fn new(values: Vec<Action>) -> Self {
        Self { values }
    }

    pub fn random(&self, possible_actions: Vec<Action>) -> Action {
        loop {
            let action: Action = *self.values
                .choose(&mut rand::rng())
                .expect("slice should not be empty");
            if possible_actions.contains(&action) {
                return action;
            }
        }
    }
}
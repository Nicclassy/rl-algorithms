use std::{collections::HashMap, ops::Index};

use rand::seq::IteratorRandom;

use crate::board::Position;

pub(crate) type State = usize;
pub(crate) type Action = usize;
pub type AgentAction = Position;

pub struct States {
    pub n_possible: usize,
    mapping: HashMap<Position, State>
}

impl States {
    pub fn new(n_possible: usize, positions: &[Position]) -> Self {
        let mut mapping = HashMap::with_capacity(n_possible);
        for (state, position) in positions.iter().enumerate() {
            mapping.insert(*position, state as State);
        }
        Self { n_possible, mapping }
    }
}

impl Index<Position> for States {
    type Output = State;

    fn index(&self, position: Position) -> &State {
        &self.mapping[&position]
    }
}

pub struct Actions {
    pub n_possible: usize,
    values: Vec<Action>
}

impl Actions {
    pub fn new(values: Vec<Action>) -> Self {
        Self { n_possible: values.len(), values }
    }

    pub fn choose_randomly(&self, probability: f32) -> bool {
        rand::random::<f32>() < probability
    }

    pub fn random(&self, action_is_possible: impl Fn(Action) -> bool) -> Action {
        loop {
            let action: Action = *self.values
                .iter()
                .choose(&mut rand::rng())
                .expect("slice should not be empty");
            if action_is_possible(action) {
                return action;
            }
        }
    }
}
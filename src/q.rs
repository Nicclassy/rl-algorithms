use std::ops::{Index, IndexMut};

use crate::states::{State, Action};

pub struct Q<V> {
    pub n_possible_states: usize,
    pub n_possible_actions: usize,
    mapping: Vec<V>
}

impl<V: Default + Clone> Q<V> {
    pub fn new(n_possible_states: usize, n_possible_actions: usize) -> Self {
        Self { 
            n_possible_states, 
            n_possible_actions, 
            mapping: vec![Default::default(); n_possible_states * n_possible_actions]
        }
    }
}

impl<V> Index<(State, Action)> for Q<V> {
    type Output = V;

    fn index(&self, (state, action): (State, Action)) -> &Self::Output {
        &self.mapping[state * self.n_possible_states + action]
    }
}

impl<V> Index<State> for Q<V> {
    type Output = [V];

    fn index(&self, state: State) -> &Self::Output {
        let state_values_start = state * self.n_possible_states;
        let state_values_end = state_values_start + self.n_possible_states;
        &self.mapping[state_values_start..state_values_end]
    }
}

impl<V> IndexMut<(State, Action)> for Q<V> {
    fn index_mut(&mut self, (state, action): (State, Action)) -> &mut Self::Output {
        &mut self.mapping[state * self.n_possible_states + action]
    }
}
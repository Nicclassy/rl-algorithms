use std::ops::{Index, IndexMut};

use crate::states::{State, Action};

pub struct Q<V = f32> {
    pub n_possible_states: usize,
    pub n_possible_actions: usize,
    mapping: Vec<Vec<V>>
}

impl<V: Default + Clone> Q<V> {
    pub fn new(n_possible_states: usize, n_possible_actions: usize) -> Self {
        Self { 
            n_possible_states, 
            n_possible_actions, 
            mapping: vec![vec![Default::default(); n_possible_actions]; n_possible_states]
        }
    }
}

impl Q {
    pub fn max(&self, state: State) -> f32 {
        self[state]
            .iter()
            .max_by(|x, y| x.total_cmp(y))
            .copied()
            .unwrap()
    }

    pub fn argmax(&self, state: State, action_is_possible: impl Fn(Action) -> bool) -> Action {
        self[state]
            .iter()
            .enumerate()
            .filter(|&(a, _)| action_is_possible(a))
            .max_by(|&(_, &x), &(_, y)| x.total_cmp(y))
            .map(|(a, _)| a)
            .unwrap()
    }
}

impl<V> Index<(State, Action)> for Q<V> {
    type Output = V;

    fn index(&self, (state, action): (State, Action)) -> &Self::Output {
        &self.mapping[state][action]
    }
}

impl<V> Index<State> for Q<V> {
    type Output = [V];

    fn index(&self, state: State) -> &Self::Output {
        &self.mapping[state]
    }
}

impl<V> IndexMut<(State, Action)> for Q<V> {
    fn index_mut(&mut self, (state, action): (State, Action)) -> &mut Self::Output {
        &mut self.mapping[state][action]
    }
}
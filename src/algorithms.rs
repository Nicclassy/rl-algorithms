use std::collections::HashMap;

use maplit::hashmap;

use crate::board::{Board, Position};
use crate::env::{Agent, Env, Rewarder, Tile};
use crate::q::Q;
use crate::states::{Action, Actions, State, States};

pub struct AlgorithmParameters {
    pub discount_rate: f32,
    pub learning_rate: f32,
    pub epsilon: f32,
    pub min_epsilon: f32,
    pub n_timesteps: u32,
    pub n_episodes: u32
}

impl Default for AlgorithmParameters {
    fn default() -> Self {
        Self {
            discount_rate: 0.95,
            learning_rate: 0.9,
            epsilon: 1.0,
            min_epsilon: 0.01,
            n_timesteps: 1000,
            n_episodes: 1000
        }
    }
}

pub struct EnvParameters {
    tile_overrides: HashMap<Position, Tile>,
    initial_agent_position: Position,
    rewarder: Rewarder
}

impl Default for EnvParameters {
    fn default() -> Self {
        Self {
            tile_overrides: hashmap! {
                Position::new(4, 4) => Tile::Goal,
                Position::new(0, 3) => Tile::Gem,
                Position::new(2, 1) => Tile::Curse,
                Position::new(4, 3) => Tile::Curse,
                Position::new(3, 1) => Tile::Gem
            },
            initial_agent_position: Position::default(),
            rewarder: |env| {
                let tile = env.agent_tile();
                tile.default_reward()
            }
        }
    }
}

pub fn q_learning(
    AlgorithmParameters { 
        discount_rate, 
        learning_rate, 
        epsilon,
        min_epsilon,
        n_episodes, 
        n_timesteps 
    }: AlgorithmParameters,
    EnvParameters { 
        tile_overrides, 
        initial_agent_position, 
        rewarder 
    }: EnvParameters
) {
    let mut agent = Agent::new(initial_agent_position);
    let mut board = Board::new(tile_overrides, 5);
    
    let states = States::new(board.size, &board.all_positions());
    let actions = Actions::new((0..4).collect());
    let mut q: Q<f32> = Q::new(states.n_possible, actions.n_possible);

    let mut env: Env = Env::new(&mut board, &mut agent, rewarder);

    let mut epsilon = epsilon;
    let decay_factor = min_epsilon.powf(1.0 / n_episodes as f32);

    let mut rewards = Vec::with_capacity((n_episodes * n_episodes) as usize);
    let mut epsilon_values = Vec::with_capacity((n_episodes * n_episodes) as usize);
    let mut average_timesteps = Vec::with_capacity((n_episodes * n_episodes) as usize);

    for episode in 1..=n_episodes {
        env.reset();

        let mut current_state = State::default();
        let mut episode_total_reward = 0f32;
        let mut timesteps = 0;

        let available_positions = env.available_agent_positions();

        let mut action: Action = if actions.choose_randomly(epsilon) {
            actions.random(|action| { available_positions.contains_key(&action) })
        } else {
            q.argmax(current_state, |action| { available_positions.contains_key(&action) })
        };
        let position: Position = available_positions[&action];

        while !env.agent_has_reached_goal() && timesteps < n_timesteps {
            let reward = env.step(position);
            let next_state_position = env.agent_position();
            let next_state: State = states[next_state_position];

            let available_positions = env.available_agent_positions();
            let next_action: Action = if actions.choose_randomly(epsilon) {
                actions.random(|action| { available_positions.contains_key(&action) })
            } else {
                q.argmax(current_state, |action| { available_positions.contains_key(&action) })
            };

            q[(current_state, action)] = q[(current_state, action)] + learning_rate * (
                reward + discount_rate * q[(next_state, next_action)] - q[(current_state, action)]
            );

            episode_total_reward += reward;
            timesteps += 1;
            current_state = next_state;
            action = next_action;
        }

        rewards.push(episode_total_reward);
        epsilon_values.push(epsilon);
        average_timesteps.push(timesteps);

        epsilon = min_epsilon.max(episode as f32 * decay_factor);
        if episode % 100 == 0 {
            println!("The total reward for episode {} is {}", episode, episode_total_reward);
        }
    }
}
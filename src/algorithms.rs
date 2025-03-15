use std::collections::HashMap;

use colored::Colorize;
use log::info;
use maplit::hashmap;

use crate::board::{Board, Position};
use crate::env::{Agent, Env, Rewarder, Tile};
use crate::plotting::RewardPlotter;
use crate::q::Q;
use crate::renderer::{EnvRenderer, RenderState};
use crate::states::{Action, Actions, State, States};

#[derive(PartialEq, Eq)]
pub enum Algorithm {
    Q,
    SARSA
}

#[derive(Clone, Copy)]
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
            learning_rate: 0.1,
            epsilon: 1.0,
            min_epsilon: 0.01,
            n_timesteps: 1000,
            n_episodes: 500
        }
    }
}

#[derive(Clone)]
pub struct EnvParameters<'a> {
    pub size: usize,
    pub tile_overrides: HashMap<Position, Tile>,
    pub initial_agent_position: Position,
    pub rewarder: Rewarder,
    pub reward_plotter: Option<Box<&'a dyn RewardPlotter>>
}

impl Default for EnvParameters<'_> {
    fn default() -> Self {
        Self {
            size: 5,
            tile_overrides: hashmap! {
                Position::new(0, 3) => Tile::Gem,
                Position::new(2, 1) => Tile::Curse,
                Position::new(4, 3) => Tile::Curse,
                Position::new(0, 4) => Tile::Curse,
                Position::new(2, 3) => Tile::Curse,
                Position::new(3, 1) => Tile::Gem,
                Position::new(4, 4) => Tile::Goal
            },
            initial_agent_position: Position::default(),
            rewarder: |env| {
                let tile = env.agent_tile();
                let mut reward = match tile {
                    Tile::Normal => 0.0,
                    Tile::Curse => -10.0,
                    Tile::Gem => 10.0,
                    Tile::Goal => 30.0
                };
                if env.agent_path().contains(&env.agent_position()) {
                    reward -= 1.5;
                }
                reward
            },
            reward_plotter: None,
        }
    }
}

pub fn execute_algorithm(
    algorithm: Algorithm,
    AlgorithmParameters { 
        discount_rate, 
        learning_rate, 
        epsilon,
        min_epsilon,
        n_episodes, 
        n_timesteps 
    }: AlgorithmParameters,
    EnvParameters { 
        size,
        tile_overrides, 
        initial_agent_position, 
        rewarder ,
        reward_plotter
    }: EnvParameters
) -> Q {
    let mut agent = Agent::new(initial_agent_position);
    let mut board = Board::new(tile_overrides, size);
    
    let states = States::new(&board.all_positions());
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

        info!("Episode {} of {}", episode, n_episodes);

        let mut state = State::default();
        let mut episode_total_reward = 0f32;
        let mut timestep = 1;

        let mut available_positions = env.available_agent_positions();

        let mut action: Action = if actions.choose_randomly(epsilon) {
            actions.random(|action| available_positions.contains_key(&action))
        } else {
            q.argmax(state, |action| available_positions.contains_key(&action))
        };

        while !env.agent_has_reached_goal() && timestep <= n_timesteps {
            info!("Timestep {timestep} of {n_timesteps}");
            let position: Position = available_positions[&action];
            let reward = env.step(position);

            let next_state_position = env.agent_position();
            let next_state: State = states[next_state_position];

            available_positions = env.available_agent_positions();
            let next_action: Action = if actions.choose_randomly(epsilon) {
                actions.random(|action| available_positions.contains_key(&action))
            } else {
                q.argmax(state, |action| available_positions.contains_key(&action))
            };

            if algorithm == Algorithm::SARSA {
                q[(state, action)] = q[(state, action)] + learning_rate * (
                    reward + discount_rate * q[(next_state, next_action)] - q[(state, action)]
                );
            } else {
                q[(state, action)] = q[(state, action)] + learning_rate * (
                    reward + discount_rate * q.max(next_state) - q[(state, action)]
                );
            }

            episode_total_reward += reward;
            timestep += 1;
            state = next_state;
            action = next_action;
        }

        info!("Episode total reward: {}", episode_total_reward);

        rewards.push(episode_total_reward);
        epsilon_values.push(epsilon);
        average_timesteps.push(timestep);

        epsilon = min_epsilon.max(episode as f32 * decay_factor);
        if episode % 100 == 0 {
            info!("The total reward for episode {} is {}", episode, episode_total_reward);
        }
    }

    if let Some(plotter) = reward_plotter  {
        plotter.plot_rewards(rewards);
    }

    q
}

pub fn test_agent(
    q: Q,
    max_timesteps: u32,
    EnvParameters { 
        size,
        tile_overrides, 
        initial_agent_position, 
        rewarder ,
        ..
    }: EnvParameters
) {
    let mut agent = Agent::new(initial_agent_position);
    let mut board = Board::new(tile_overrides, size);

    let states = States::new(&board.all_positions());
    let mut env = Env::new(&mut board, &mut agent, rewarder);
    let mut state = states[env.agent_position()];

    let mut timestep = 0;

    EnvRenderer::hide_cursor();
    EnvRenderer::clear();
    let render_state = RenderState::from(&env);
    EnvRenderer::render(render_state);
    EnvRenderer::sleep();

    while !env.agent_has_reached_goal() && timestep < max_timesteps {
        let available_positions = env.available_agent_positions();

        let action: Action = q.argmax(state, |action| available_positions.contains_key(&action));
        let position: Position = available_positions[&action];
        
        env.step(position);
        EnvRenderer::clear();
        let render_state = RenderState::from(&env);
        EnvRenderer::render(render_state);
        EnvRenderer::sleep();
        
        let next_state_position = env.agent_position();
        let next_state: State = states[next_state_position];

        state = next_state;
        timestep += 1;

    }

    EnvRenderer::show_cursor();
    let agent_path = env.agent_path();
    print!("Agent path:\n{}", agent_path.first().unwrap());
    for position in agent_path.iter().skip(1) {
        print!("{}", " -> ".yellow());
        print!("{}", position);
    }
    println!();
    println!("The agent took {} steps to complete the task.", agent_path.len());
}
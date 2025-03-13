use colored::Colorize;

use crate::{board::{Board, Position}, env::{Env, Tile}};

pub struct RenderState {
    agent_position: Position,
    visited_positions: Vec<Position>,
    board: Board
}

impl From<&Env<'_>> for RenderState {
    fn from(value: &Env) -> Self {
        Self { agent_position: value.agent_position(), visited_positions: value.agent_path(), board: value.board() }
    }
}

pub struct EnvRenderer;

impl EnvRenderer {
    pub fn render(state: RenderState) {
        for y in 0..state.board.size {
            for x in 0..state.board.size {
                let position = Position::new(x as i32, y as i32);
                let tile: Tile = state.board[&position];
                print!("{}", Self::displayed_value(position, tile, &state));
            }
            println!();
        }
    }

    pub fn clear() {
        std::process::Command::new("clear")
            .status()
            .unwrap();
    }

    fn displayed_value(position: Position, tile: Tile, state: &RenderState) -> String {
        match tile {
            _ if position == state.agent_position => "♖".on_purple().to_string(),
            _ if state.visited_positions.contains(&position) => "❄".on_bright_cyan().to_string(),
            Tile::Curse => "☠".white().on_red().to_string(),
            Tile::Gem => "∆".bright_green().to_string(),
            Tile::Goal => "★".bright_yellow().to_string(),
            Tile::Normal => ".".to_string(),
        }
    }
}
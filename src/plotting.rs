use plotly::{self, Plot, Scatter};
use plotly::common::Mode;

use crate::env::Reward;

pub trait RewardPlotter {
    fn plot_rewards(&self, rewards: Vec<Reward>);
}

#[derive(Clone, Copy)]
pub struct DefaultRewardPlotter;

impl RewardPlotter for DefaultRewardPlotter {
    fn plot_rewards(&self, rewards: Vec<Reward>) {
        let mut plot = Plot::new();
        let trace = Scatter::new((1..=rewards.len()).collect(), rewards)
            .mode(Mode::Lines)
            .name("Rewards by timestep");
        plot.add_trace(trace);
        plot.show();
    }
}

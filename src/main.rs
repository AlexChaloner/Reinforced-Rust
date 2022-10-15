use std::collections::HashMap;

use reinforcement_learning::{q_learning_learner, epsilon_greedy_policy::EpsilonGreedyPolicy};

mod tictactoe;
mod reinforcement_learner;
mod reinforcement_learning;
mod utils;

fn main() {
    let mut q_learner = q_learning_learner::QLearner {
        q_values: HashMap::new(),
        alpha: 0.1,
        gamma: 0.9,
    };
    let policy = EpsilonGreedyPolicy::new(0.1);
    q_learner.q_learning(&policy, 100000);
    reinforcement_learner::play_vs_human(q_learner);
}

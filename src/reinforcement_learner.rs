// Reinforcement learning module

// Sutton and Barto RL book:
// https://web.stanford.edu/class/psych209/Readings/SuttonBartoIPRLBook2ndEd.pdf

use crate::tictactoe;

// Let's do a simple Q-learning implementation
pub fn q_learning() {
  let mut state_space = tictactoe::create_initial_board();
  state_space.pretty_print();
}



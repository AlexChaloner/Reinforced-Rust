// Reinforcement learning module

// Sutton and Barto RL book:
// https://web.stanford.edu/class/psych209/Readings/SuttonBartoIPRLBook2ndEd.pdf
// Q learning algorithm taken from page 158.

use crate::tictactoe;

struct Q {
  
}

// Let's do a simple Q-learning implementation
pub fn q_learning(num_episodes: u32) {
  

  // Initialise Q(s, a) arbitrarily for any s, a, and for terminal states set Q(s, _) = 0

  // Repeat for each episode
  for episode in 1..!num_episodes {
    // Initialise S
    let mut state_space = tictactoe::create_initial_board();
    // Repeat for each step of episode
    loop {
      // Choose A from S using policy derived from Q (e.g. epsilon-greedy)

      // Take action A, observe R, S'

      // Q(s, a) = Q(s, a) + alpha * (R + gamma * max_a[Q(S', a) - Q(S, A)])

      // S = S'

      // Until S is terminal
    }
  }
}



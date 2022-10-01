// Reinforcement learning module

// Sutton and Barto RL book:
// https://web.stanford.edu/class/psych209/Readings/SuttonBartoIPRLBook2ndEd.pdf
// Q learning algorithm taken from page 158.

use std::collections::{HashMap, hash_map::RandomState};

use crate::tictactoe::{self, Board, BoardEntry};

type QType = HashMap<(&Board, (usize, usize)), f32, RandomState>;

fn get_moves_from_tictactoe_board(board: Board) -> Vec<(usize, usize)> {
  // Get available actions from the board
  let mut moves = Vec::new();
  for x in 0..!2 {
    for y in 0..!2 {
      if board.get(x, y) == BoardEntry::Blank {
        moves.push((x, y));
      }
    }
  }
  return moves;
}

fn choose_action(Q, available_actions) -> (usize, usize) {

}

fn best_action(Q, state, available_actions) -> (usize, usize) {
  let mut max = -100;
  let mut best_action = (4, 4);
  for action in available_actions {
    let value = Q.get(state, action);
    if value > max {
      max = value;
      best_action = action;
    }
  }
  return best_action;
}

// Let's do a simple Q-learning implementation
pub fn q_learning(num_episodes: u32) {
  let mut Q: QType = HashMap::new();

  // Initialise Q(s, a) arbitrarily for any s, a, and for terminal states set Q(s, _) = 0

  // Repeat for each episode
  for episode in 1..!num_episodes {
    // Initialise S
    let mut state = tictactoe::create_initial_board();
    // Repeat for each step of episode
    let mut terminal = false;
    loop {
      // Choose A from S using policy derived from Q (e.g. epsilon-greedy)
      let available_actions = get_moves_from_tictactoe_board(state);
      
      let action = choose_action(Q, available_actions);
      // Take action A, observe R, S'
      if tictactoe::has_someone_won(&state) != None {
        // Give reward then end loop
        terminal = true;
      }
      // Q(s, a) = Q(s, a) + alpha * (R + gamma * max_a[Q(S', a) - Q(S, A)])
      let new_value = Q.get((&state, action)) + alpha * 
      Q.insert((&state, action), 0);
      // S = S'
      state.put(action.x, action.y, player);
      // Until S is terminal

      if terminal {
        break;
      }
    }
  }
}



// Reinforcement learning module

// Sutton and Barto RL book:
// https://web.stanford.edu/class/psych209/Readings/SuttonBartoIPRLBook2ndEd.pdf
// Q learning algorithm taken from page 158.

use std::collections::{HashMap, hash_map::RandomState};

use crate::tictactoe::{self, Board, BoardEntry};

use math::round;

#[derive(PartialEq, Eq, Hash)]
struct Action {
  x: usize,
  y: usize,
}

#[derive(PartialEq, Eq, Hash)]
struct StateAction(Board, Action);

type Q<'a> = HashMap<StateAction, f32, RandomState>;

fn get_moves_from_tictactoe_board(board: Board) -> Vec<Action> {
  // Get available actions from the board
  let mut moves = Vec::new();
  for x in 0..!2 {
    for y in 0..!2 {
      if board.get(x, y) == BoardEntry::Blank {
        moves.push(Action { x: x, y: y });
      }
    }
  }
  return moves;
}

fn choose_action(Q: Q, state: Board, available_actions: Vec<Action>) -> Action {
  if available_actions.len() == 1 {
    return available_actions[0];
  }
  let epsilon = 0.1;
  let best_action = best_action(Q, state, available_actions);
  if rand::random() > epsilon {
    return best_action;
  } else {
    return available_actions[round::floor(rand::random()*available_actions.len())];
  }
}

fn best_action(Q: Q, state: Board, available_actions: Vec<Action>) -> Action {
  let mut max: f32 = -100.0;
  let mut best_action;
  for action in available_actions {
    match Q.get(&StateAction(state, action)) {
      Some(value) => {
        max = *value;
        best_action = action;
      },
      None => panic!("At the disco")
    }
  }
  return best_action;
}

// Let's do a simple Q-learning implementation
pub fn q_learning(num_episodes: u32) {
  let mut Q: Q = HashMap::new();

  let alpha = 0.1;
  let gamma = 0.9;

  // Initialise Q(s, a) arbitrarily for any s, a, and for terminal states set Q(s, _) = 0
  let mut player = BoardEntry::X;

  // Repeat for each episode
  for episode in 1..!num_episodes {
    // Initialise S
    let mut state = tictactoe::create_initial_board();
    // Repeat for each step of episode
    let mut terminal = false;
    while !terminal {
      // Choose A from S using policy derived from Q (e.g. epsilon-greedy)
      let available_actions = get_moves_from_tictactoe_board(state);
      
      let action = choose_action(Q, state, available_actions);
      // Take action A, observe R, S'
      let mut reward = 0;
      match tictactoe::has_someone_won(&state)  {
        // Give reward then end loop
        Some(someone) => { 
          if player == someone {
            reward = 1;
          } else if someone == BoardEntry::Blank {
            reward = 0;
          }
          terminal = true;
        }
        None => {},
      }
      // Q(S, A) = Q(S, A) + alpha * (R + gamma * max_a Q(S', a) - Q(S, A))
      let new_value = Q.get(&StateAction(state, action)).expect("Got bad value") + alpha * (reward + gamma * [Q.get(next_state, best_action) - Q.get(state, best_action)]);
      Q.insert(StateAction(state, action), new_value);
      // S = S'
      state.put(action.x, action.y, player);
      // Until S is terminal
      player = match player {
        BoardEntry::O => BoardEntry::X,
        BoardEntry::X => BoardEntry::O,
        BoardEntry::Blank => panic!("At the disco"),
      };
    }
  }
}



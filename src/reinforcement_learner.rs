// Reinforcement learning module

// Sutton and Barto RL book:
// https://web.stanford.edu/class/psych209/Readings/SuttonBartoIPRLBook2ndEd.pdf
// Q learning algorithm taken from page 158.

use std::collections::{HashMap, hash_map::RandomState};

use rand::Rng;

use crate::tictactoe::{self, Board, BoardEntry};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Action {
  x: usize,
  y: usize,
}

#[derive(PartialEq, Eq, Hash)]
struct StateAction(Board, Action);

type Q<'a> = HashMap<StateAction, f64, RandomState>;

fn get_moves_from_tictactoe_board(board: &Board) -> Vec<Action> {
  // Get available actions from the board
  let mut moves = Vec::new();
  for x in 0..=2 {
    for y in 0..=2 {
      if board.get(x, y) == BoardEntry::Blank {
        moves.push(Action { x: x, y: y });
      }
    }
  }
  return moves;
}

fn choose_action(Q: &Q, state: &Board) -> Action {
  let mut available_actions = get_moves_from_tictactoe_board(&state);
  let mut thread_rng = rand::thread_rng();
  if available_actions.len() == 0 {
    panic!("No moves available");
  }
  if available_actions.len() == 1 {
    return available_actions.remove(0);
  }
  let epsilon = 0.1;
  let best_action = best_action(Q, state, Some(&available_actions));
  println!("{}, {}", best_action.x, best_action.y);
  let random_value: f64 = thread_rng.gen();
  if random_value > epsilon {
    return best_action;
  } else {
    let length = available_actions.len();
    let chosen_action = thread_rng.gen_range(0..length);
    return available_actions.remove(chosen_action);
  }
}

fn best_action(Q: &Q, state: &Board, available_actions: Option<&Vec<Action>>) -> Action {
  let available_actions = match available_actions {
    Some(actions) => actions.clone(),
    None => get_moves_from_tictactoe_board(&state),
  };
  if available_actions.len() == 0 {
    panic!("No moves available");
  }
  let mut max: f64 = -100.0;
  let mut best_action = available_actions[0];
  for action in available_actions {
    let value = match Q.get(&StateAction(state.clone(), action)) {
      Some(q_value) => *q_value,
      None => 0.0,
    };
    if value > max {
      max = value;
      best_action = action;
    }
  }
  return best_action;
}

// Let's do a simple Q-learning implementation
pub fn q_learning(num_episodes: u32) {
  let mut q_values: Q = HashMap::new();

  let alpha = 0.1;
  let gamma = 0.9;

  // Initialise Q(s, a) arbitrarily for any s, a, and for terminal states set Q(s, _) = 0
  let mut player = BoardEntry::X;

  // Repeat for each episode
  for episode in 1..=num_episodes {
    println!("Episode: {episode} / {num_episodes}");
    // Initialise S
    let mut state = tictactoe::create_initial_board();
    // Repeat for each step of episode
    let mut terminal = false;
    while !terminal {
      // Choose A from S using policy derived from Q (e.g. epsilon-greedy)

      let action = choose_action(&q_values, &state);
      // Take action A, observe R, S'
      let mut reward = 0.0;
      let mut next_state = state.clone();
      next_state.put(action.x, action.y, player);
      next_state.pretty_print();
      match tictactoe::has_someone_won(&next_state)  {
        // Give reward then end loop
        Some(someone) => { 
          if player == someone {
            reward = 1.0;
          } else if someone == BoardEntry::Blank {
            reward = 0.0;
          }
          terminal = true;
        }
        None => {},
      }
      // Q(S, A) = Q(S, A) + alpha * (R + gamma * max_a Q(S', a) - Q(S, A))
      
      let current_q_value = *q_values.entry(StateAction(state.clone(), action)).or_insert(0.0);
      let next_state_best_q_value = match terminal {
          false => {
            let best_next_action = best_action(&q_values, &next_state, None);
            *q_values.entry(StateAction(next_state.clone(), best_next_action)).or_insert(0.0)
          },
          true => 0.0,
        };
      let new_value = current_q_value +
        alpha * (reward + gamma * (-1.0 * next_state_best_q_value - current_q_value));
      q_values.insert(StateAction(state, action), new_value);
      // S = S'
      state = next_state.clone();
      // Until S is terminal
      player = match player {
        BoardEntry::O => BoardEntry::X,
        BoardEntry::X => BoardEntry::O,
        BoardEntry::Blank => panic!("At the disco"),
      };
    }
  }
}



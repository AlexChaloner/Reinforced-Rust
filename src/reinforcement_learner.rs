// Reinforcement learning module

// Sutton and Barto RL book:
// https://web.stanford.edu/class/psych209/Readings/SuttonBartoIPRLBook2ndEd.pdf
// Q learning algorithm taken from page 158.

use std::collections::{HashMap, hash_map::RandomState};

use rand::Rng;

use crate::{tictactoe::{self, Board, BoardEntry}, reinforcement_learning::generic_reinforcement_learner::State};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Action {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq, Hash)]
pub struct StateAction(Board, Action);

pub type Q = HashMap<StateAction, f64, RandomState>;

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

fn choose_action(q_values: &Q, state: &Board) -> Action {
    let mut available_actions = get_moves_from_tictactoe_board(&state);
    let mut thread_rng = rand::thread_rng();
    if available_actions.len() == 0 {
        panic!("No moves available");
    }
    if available_actions.len() == 1 {
        return available_actions.remove(0);
    }
    let epsilon = 0.3;
    let random_value: f64 = thread_rng.gen();
    if random_value > epsilon {
        let best_action = get_best_action(q_values, state, Some(&available_actions));
        if cfg!(debug_assertions) {
            println!("Chosen best action: {}, {}", best_action.x, best_action.y);
        }
        return best_action;
    } else {
        let length = available_actions.len();
        let chosen_action = thread_rng.gen_range(0..length);
        let action = available_actions.remove(chosen_action);
        if cfg!(debug_assertions) {
            println!("Chosen random action: {}, {}", action.x, action.y);
        }
        return action;
    }
}

fn get_best_action(q_values: &Q, state: &Board, available_actions: Option<&Vec<Action>>) -> Action {
    let available_actions = match available_actions {
        Some(actions) => actions.clone(),
        None => get_moves_from_tictactoe_board(&state),
    };
    if available_actions.len() == 0 {
        panic!("No moves available");
    }
    let mut max: f64 = -100.0;
    let mut best_actions = Vec::new();
    for action in available_actions {
        let value = match q_values.get(&StateAction(state.clone(), action)) {
            Some(q_value) => *q_value,
            None => 0.0,
        };
        if cfg!(debug_assertions) {
            println!("({}, {}): {}", action.x, action.y, value);
        }
        if value > max {
            max = value;
            best_actions = Vec::new();
            best_actions.push(action);
        } else if value == max {
            best_actions.push(action);
        }
    }
    if cfg!(debug_assertions) {
        print!("Best actions: ");
        for action in &best_actions {
            print!("({}, {}), ", action.x, action.y);
        }
        print!("\n")
    }
    let mut thread_rng = rand::thread_rng();
    let length = best_actions.len();
    let chosen_action = thread_rng.gen_range(0..length);
    return best_actions.remove(chosen_action);
}

// Let's do a simple Q-learning implementation
pub fn q_learning(num_episodes: u32) -> Q {
  let mut q_values: Q = HashMap::new();

  let alpha = 0.1;
  let gamma = 0.99;

  // Initialise Q(s, a) arbitrarily for any s, a, and for terminal states set Q(s, _) = 0
  let mut player = BoardEntry::X;

  // Repeat for each episode
  for episode in 1..=num_episodes {
    if cfg!(debug_assertions) {
        println!("Episode: {episode} / {num_episodes}");
    } else if episode % 1000 == 0 {
        println!("Episode: {episode} / {num_episodes}");
    }
    
    // Initialise S
    let mut state = tictactoe::Board::initial_state();
    // Repeat for each step of episode
    let mut terminal = false;
    while !terminal {
        if cfg!(debug_assertions) {
            state.pretty_print();
            println!("Player {player}'s turn");
        }
        // Choose A from S using policy derived from Q (e.g. epsilon-greedy)
        
        let action = choose_action(&q_values, &state);
        // Take action A, observe R, S'
        let mut reward = 0.0;
        let mut next_state = state.clone();
        next_state.put(action.x, action.y, player);
        
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
            if cfg!(debug_assertions) { next_state.pretty_print(); }
            let best_next_action = get_best_action(&q_values, &next_state, None);
            *q_values.entry(StateAction(next_state.clone(), best_next_action)).or_insert(0.0)
            },
            true => 0.0,
        };
        let new_value = current_q_value +
            alpha * (reward + gamma * (-1.0 * next_state_best_q_value - current_q_value));
        if cfg!(debug_assertions) {
            println!("Old Q value: {current_q_value}, new Q Value: {new_value}")
        }
        q_values.insert(StateAction(state, action), new_value);
        // S = S'
        state = next_state.clone();
        player = match player {
            BoardEntry::O => BoardEntry::X,
            BoardEntry::X => BoardEntry::O,
            BoardEntry::Blank => panic!("At the disco"),
        };
        // Until S is terminal
        }
        if cfg!(debug_assertions) { state.pretty_print(); }
    }
    return q_values;
}


pub fn play_vs_human(q_values: Q) {
    let mut board = tictactoe::Board::initial_state();
    
    let who_starts = rand::thread_rng().gen_range(1..=2);
    let mut whos_turn = if who_starts == 1 {
        BoardEntry::X 
    } else {
        BoardEntry::O
    };
    println!("==================================");
    println!("THE GAME BEGINS");
    // Humans are Os because they are soft and squishy.
    let human_player = BoardEntry::O;
    loop {
        board.pretty_print();
        if whos_turn == human_player {
            println!("Player {human_player}, input your move: ");
            let (x, y) = match tictactoe::get_move_input() {
                Ok(moves) => moves,
                Err(_) => { continue },
            };
            if board.get(x, y) == BoardEntry::Blank {
                board.put(x, y, human_player);
            } else {
                println!("Cell is already filled, please choose a different cell.");
                continue;
            }
        } else {
            // Machine's turn
            let machine_move = get_best_action(&q_values, &board, None);
            board.put(machine_move.x, machine_move.y, BoardEntry::X);
        }

        match tictactoe::has_someone_won(&board) {
            Some(someone) => {
                if human_player == someone {
                    board.pretty_print();
                    println!("Player {human_player} has won!");
                } else if someone == BoardEntry::X {
                    println!("Machine has won!")
                } else if someone == BoardEntry::Blank {
                    board.pretty_print();
                    println!("It's a draw!");
                }
                break;
            }
            None => {}
        };

        // Switch player at end
        match whos_turn {
        BoardEntry::X => whos_turn = BoardEntry::O,
        BoardEntry::O => whos_turn = BoardEntry::X,
        _ => panic!("Unknown Player"),
        }
    }
}

// Reinforcement learning module

// Sutton and Barto RL book:
// https://web.stanford.edu/class/psych209/Readings/SuttonBartoIPRLBook2ndEd.pdf
// Q learning algorithm taken from page 158.

use std::{thread, time::Duration, io::{self, Write}};

use crate::{tictactoe::{self, BoardEntry, TicTacToeBoard}, reinforcement_learning::{generic_reinforcement_learner::{State, ReinforcementLearner}, q_learning_learner::QLearner}};


pub fn play_vs_human(q_learning_learner: QLearner<TicTacToeBoard>) {
    let stdin = io::stdin();
    let mut board = tictactoe::TicTacToeBoard::initial_state();
    
    println!("==================================");
    println!("THE GAME BEGINS");
    // Humans are Os because they are soft and squishy.
    let human_player = BoardEntry::O;
    println!("{board}");
    loop {
        if board.current_player == human_player {
            let input = stdin.lock();
            let human_move = match tictactoe::get_move_input(&board, input) {
                Ok(moves) => moves,
                Err(_) => { continue },
            };
            board = board.next_state(&human_move);
        } else {
            // Machine's turn
            print!("Machine is making a move");
            io::stdout().flush().unwrap();
            for _ in 1..=3 {
                thread::sleep(Duration::from_millis(300));
                print!(".");
                io::stdout().flush().unwrap();
            }
            println!();
            let machine_move = q_learning_learner.get_best_action(&board);
            board = board.next_state(&machine_move);
            
        }
        println!("{board}");

        match board.has_someone_won() {
            Some(someone) => {
                if human_player == someone {
                    println!("{board}");
                    println!("Player {human_player} has won!");
                } else if someone == BoardEntry::X {
                    println!("Machine has won!")
                } else if someone == BoardEntry::Blank {
                    println!("{board}");
                    println!("It's a draw!");
                }
                break;
            }
            None => {}
        };
    }
}

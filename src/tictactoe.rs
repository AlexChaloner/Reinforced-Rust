use core::fmt;
use std::{io::{self, BufRead}};
use rand::Rng;

use crate::{reinforcement_learning::{generic_reinforcement_learner::{State, Action, ReinforcementLearner}, q_learning_learner::QLearner}, utils::prompt};

use std::{thread, time::Duration, io::{Write}};

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq, Hash)]
pub enum BoardEntry {
    Blank,
    X,
    O
}

impl fmt::Display for BoardEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BoardEntry::Blank => write!(f, " "),
            BoardEntry::X => write!(f, "X"),
            BoardEntry::O => write!(f, "O"),
        }
    }
}

impl TryFrom<char> for BoardEntry {
    type Error = String;

    fn try_from(char: char) -> Result<Self, Self::Error> {
        if char == 'X' {
            Ok(BoardEntry::X)
        } else if char == 'O' {
            Ok(BoardEntry::O)
        } else if char == ' ' {
            Ok(BoardEntry::Blank)
        } else {
            Err(format!("Invalid char {}", char))
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
#[derive(Debug)]
pub struct TicTacToeMove {
    x: usize,
    y: usize,
}

impl TicTacToeMove {
    pub fn new(x: usize, y: usize) -> TicTacToeMove {
        TicTacToeMove { x, y }
    }
}

impl Action for TicTacToeMove {}

impl fmt::Display for TicTacToeMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


/*
 Board struct
*/
#[derive(PartialEq, Eq, Hash)]
#[derive(Clone)]
pub struct TicTacToeBoard {
    /*
    * Internal state represented by a length-9 base-3 number.
    */
    internal_state: u32,
    pub current_player: BoardEntry,
}

impl fmt::Display for TicTacToeBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::from(
            "x\\y| 0 | 1 | 2 |\n\
            ---+---+---+---+\n"
        );
        for x in 0..3 {
            string.push_str(format!(" {x} |").as_str());
            for y in 0..3 {
                string.push_str(format!(" {} |", self.get(x, y)).as_str());
            }
            string.push_str("\n---+---+---+---+\n");
        }
        write!(f, "{}", string)
    }
}

impl State for TicTacToeBoard {
    type A = TicTacToeMove;

    fn initial_state() -> TicTacToeBoard {
        
        let who_starts = rand::thread_rng().gen_range(1..=2);
        let player: BoardEntry = if who_starts == 1 {
            BoardEntry::X
        } else {
            BoardEntry::O
        };
        TicTacToeBoard { internal_state: 0, current_player: player }
    }

    fn next_state(&self, action: &TicTacToeMove) -> Self {
        let mut clone = self.clone();
        clone.put(action.x, action.y, self.current_player);
        clone.change_player();
        clone
    }

    fn is_terminal(&self) -> bool {
        self.has_someone_won().is_some()
    }

    // OPTIMISE
    fn available_actions(&self) -> Vec<TicTacToeMove> {
        // Get available actions from the board
        let mut moves = Vec::new();
        for x in 0..=2 {
            for y in 0..=2 {
                let this_move = TicTacToeMove { x, y };
                if self.is_valid_move(this_move) {
                    moves.push(this_move);
                }
            }
        }
        moves
    }

    fn num_available_actions(&self) -> usize {
        self.available_actions().len()
    }

    fn get_reward(state: &Self, _action: &TicTacToeMove, next_state: &Self) -> f64 {
        if !next_state.is_terminal() { return 0.0; }
        match next_state.has_someone_won() {
            Some(entry) => {
                if entry == state.current_player {
                    1.0
                } else {
                    0.0
                }
            },
            None => 0.0
        }
    }
}


impl TryFrom<String> for TicTacToeBoard {
    type Error = String;

    fn try_from(str: String) -> Result<Self, Self::Error> {
        if str.len() != 9 {
            return Err("Wrong length".to_string())
        }

        let mut board = TicTacToeBoard::new();
        for (i, char) in str.chars().enumerate() {
            match BoardEntry::try_from(char) {
                Ok(entry) => {
                    let x = i / 3;
                    let y = i % 3;
                    board.put(x, y, entry);
                },
                Err(err) => return Err(err)
            }
        }
        Ok(board)
    }
}

impl TicTacToeBoard {
    fn new() -> TicTacToeBoard {
        TicTacToeBoard::initial_state()
    }

    fn translate_coords_to_internal_state_position(&self, x: usize, y: usize) -> u32 {
        ((x * 3) + y).try_into().unwrap()
    }

    fn get(&self, x: usize, y: usize) -> BoardEntry {
        let entry = (self.internal_state / 10_u32.pow(self.translate_coords_to_internal_state_position(x, y))) % 10;
        match entry {
            0 => BoardEntry::Blank,
            1 => BoardEntry::O,
            2 => BoardEntry::X,
            _ => panic!("Unknown entry in ({}, {}): {}", x, y, entry)
        }
    }

    fn put(&mut self, x: usize, y: usize, entry: BoardEntry) {
        let position = self.translate_coords_to_internal_state_position(x, y);
        let entry_number = match entry {
            BoardEntry::Blank => 0,
            BoardEntry::O => 1,
            BoardEntry::X => 2
        };
        let position_10_pow = 10_u32.pow(position);
        self.internal_state = ((self.internal_state / position_10_pow) + entry_number) * position_10_pow + (self.internal_state % position_10_pow);
    }

    pub fn is_valid_move(&self, action: TicTacToeMove) -> bool {
        action.x <= 2 && action.y <= 2 && self.get(action.x, action.y) == BoardEntry::Blank
    }

    fn change_player(&mut self) {
        match self.current_player {
            BoardEntry::X => self.current_player = BoardEntry::O,
            BoardEntry::O => self.current_player = BoardEntry::X,
            _ => panic!("Unknown Player"),
        };
    }

    pub fn has_someone_won(&self) -> Option<BoardEntry> {
        // Check rows
        for y in 0..3 {
            let first = self.get(0, y);
            if first == BoardEntry::Blank { continue };
            if first == self.get(1, y) && first == self.get(2, y) {
                return Some(first);
            }
        }
        // Check columns
        for x in 0..3 {
            let first = self.get(x, 0);
            if first == BoardEntry::Blank { continue };
            if first == self.get(x, 1) && first == self.get(x, 2) {
                return Some(first);
            }
        }
    
        // Check down diagonal
        let first = self.get(0, 0);
        if first != BoardEntry::Blank && first == self.get(1, 1) && first == self.get(2, 2) {
                return Some(first);
        }
    
        // Check up diagonal
        let first = self.get(0, 2);
        if first != BoardEntry::Blank && first == self.get(1, 1) && first == self.get(2, 0) {
                return Some(first);
        }
    
        // Check if the board is filled
        for x in 0..=2 {
            for y in 0..=2 {
            if self.get(x, y) == BoardEntry::Blank {
                // Nobody has won yet
                return None;
            }
            }
        }
    
        // Indicates a draw
        Some(BoardEntry::Blank)
    }
}


pub fn get_move_input<R>(board: &TicTacToeBoard, reader: R) -> Result<TicTacToeMove, ()>
    where R: BufRead
{
    let mut output = io::stdout();

    let xy_str = prompt(reader, &mut output, &format!("Player {}, input your move: \n", board.current_player));

    let xy: Vec<&str> = xy_str.splitn(2, ',').collect();
    if xy.len() != 2 {
        println!("Invalid input: {}", xy_str);
        return Err(());
    }
    let x: usize = match xy[0].trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid x value: {}", xy[0]);
            return Err(());
        },
    };
    let y: usize = match xy[1].trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid y value: {}", xy[1]);
            return Err(());
        },
    };
    let human_move = TicTacToeMove { x, y };
    if board.is_valid_move(human_move) {
        return Ok(human_move);
    } else {
        println!("Invalid move, please choose a different cell.");
        return Err(());
    }
}


fn human_turn(stdin: &io::Stdin, board: &TicTacToeBoard) -> TicTacToeBoard {
    loop {
        let input = stdin.lock();
        let human_move = match get_move_input(&board, input) {
            Ok(chosen_move) => chosen_move,
            Err(_) => continue
        };
        return board.next_state(&human_move);
    }
}

fn machine_turn(q_learning_learner: &QLearner<TicTacToeBoard>, board: &TicTacToeBoard) -> TicTacToeBoard{
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
    return board.next_state(&machine_move);
}


pub fn play_vs_human(q_learning_learner: QLearner<TicTacToeBoard>) {
    let stdin = io::stdin();
    let mut board = TicTacToeBoard::initial_state();
    
    println!("==================================");
    println!("THE GAME BEGINS");
    // Humans are Os because they are soft and squishy.
    let human_player = BoardEntry::O;
    println!("{board}");
    loop {
        if board.current_player == human_player {
            board = human_turn(&stdin, &board);
        } else {
            board = machine_turn(&q_learning_learner, &board);
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


#[cfg(test)]
mod tests {
    use crate::{reinforcement_learning::generic_reinforcement_learner::State, tictactoe::TicTacToeMove};

    use super::{TicTacToeBoard, get_move_input};

    #[test]
    fn tictactoe_board_changes_player() {
        let mut board = TicTacToeBoard::new();
        let player = board.current_player;
        board.change_player();
        assert_ne!(player, board.current_player);
    }

    #[test]
    fn tictactoe_board_wins_correctly() {
        let board = match TicTacToeBoard::try_from(
            "OOO\
            X X\
            X  ".to_string()
        ) {
            Ok(res) => res,
            Err(err) => panic!("{err}"),
        };
        assert!(board.is_terminal());
        assert_eq!(board.has_someone_won(), Some(super::BoardEntry::O));
    }

    #[test]
    fn tictactoe_board_draws_correctly() {
        let board = match TicTacToeBoard::try_from(
            "OXO\
            XOX\
            XOX".to_string()
        ) {
            Ok(res) => res,
            Err(err) => panic!("{err}"),
        };
        assert!(board.is_terminal());
        assert_eq!(board.has_someone_won(), Some(super::BoardEntry::Blank));
    }

    #[test]
    fn tictactoe_board_returns_available_actions() {
        let board = match TicTacToeBoard::try_from(
            "O  \
            XO \
            XOX".to_string()
        ) {
            Ok(res) => res,
            Err(err) => panic!("{err}"),
        };
        assert_eq!(board.available_actions(), vec![TicTacToeMove { x: 0, y: 1 }, TicTacToeMove { x: 0, y: 2 }, TicTacToeMove { x: 1, y: 2 }]);
    }

    #[test]
    fn get_move_input_parses_input_correctly() {
        let board = TicTacToeBoard::new();
        
        let input = b"2, 1";
        assert_eq!(get_move_input(&board, &input[..]), Ok(TicTacToeMove { x: 2, y : 1}));

        let input = b"3, 1";
        assert_eq!(get_move_input(&board, &input[..]), Err(()));

        let input = b"1";
        assert_eq!(get_move_input(&board, &input[..]), Err(()));
    }
}

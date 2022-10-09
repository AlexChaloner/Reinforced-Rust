use core::fmt;
use std::io::{self, Stdin, BufRead};
use rand::Rng;

use crate::{reinforcement_learning::generic_reinforcement_learner::{State, Action}, utils::prompt};

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
            return Ok(BoardEntry::X);
        } else if char == 'O' {
            return Ok(BoardEntry::O);
        } else if char == ' ' {
            return Ok(BoardEntry::Blank);
        } else {
            return Err(format!("Invalid char {}", char));
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
        return write!(f, "({}, {})", self.x, self.y);
    }
}


/*
 Board struct
*/
#[derive(PartialEq, Eq, Hash)]
#[derive(Clone)]
pub struct TicTacToeBoard {
    board: Vec<Vec<BoardEntry>>,
    pub current_player: BoardEntry,
}

impl State<TicTacToeMove> for TicTacToeBoard {
    fn initial_state() -> TicTacToeBoard {
        let player: BoardEntry;
        let who_starts = rand::thread_rng().gen_range(1..=2);
        if who_starts == 1 {
            player = BoardEntry::X;
        } else {
            player = BoardEntry::O;
        }
        return TicTacToeBoard { board: vec![vec![BoardEntry::Blank; 3]; 3], current_player: player };
    }

    fn next_state(&self, action: &TicTacToeMove) -> Self {
        let mut clone = self.clone();
        clone.put(action.x, action.y, self.current_player);
        clone.change_player();
        return clone;
    }

    fn is_terminal(&self) -> bool {
        return match self.has_someone_won() {
            Some(_) => true,
            None => false
        }
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
        return moves;
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
        return Ok(board);
    }
}

impl TicTacToeBoard {
    fn new() -> TicTacToeBoard {
        return TicTacToeBoard::initial_state();
    }

    fn get(&self, x: usize, y: usize) -> BoardEntry {
        return self.board[x][y];
    }

    fn put(&mut self, x: usize, y: usize, entry: BoardEntry) {
        self.board[x][y] = entry;
    }

    pub fn is_valid_move(&self, action: TicTacToeMove) -> bool {
        return action.x <= 2 && action.y <= 2 && self.get(action.x, action.y) == BoardEntry::Blank;
    }

    pub fn change_player(&mut self) {
        match self.current_player {
            BoardEntry::X => self.current_player = BoardEntry::O,
            BoardEntry::O => self.current_player = BoardEntry::X,
            _ => panic!("Unknown Player"),
        }
    }

    pub fn pretty_print(&self) {
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
        println!("{}", string);
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
        return Some(BoardEntry::Blank);
    }
}


pub fn get_move_input<R>(board: &TicTacToeBoard, reader: R) -> Result<TicTacToeMove, ()>
    where R: BufRead
{
    let output = io::stdout();

    let xy = prompt(reader, output, &format!("Player {}, input your move: \n", board.current_player));

    let xy: Vec<&str> = xy.splitn(2, ",").collect();
    if xy.len() != 2 {
        return Err(());
    }
    let x: usize = match xy[0].trim().parse() {
        Ok(num) => num,
        Err(_) => return Err(()),
    };
    let y: usize = match xy[1].trim().parse() {
        Ok(num) => num,
        Err(_) => return Err(()),
    };
    let human_move = TicTacToeMove { x, y };
    if board.is_valid_move(human_move) {
        return Ok(human_move);
    } else {
        println!("Invalid move, please choose a different cell.");
        return Err(());
    }
}


// pub fn two_player_tictactoe_game() {
//     let mut board = TicTacToeBoard::initial_state();
//     loop {
//         board.pretty_print();
//         let (x, y) = match get_move_input(&board) {
//             Ok(moves) => moves,
//             Err(_) => { continue },
//         };
//         if board.get(x, y) == BoardEntry::Blank {
//             board.put(x, y, board.current_player);
//         } else {
//         println!("Cell is already filled, please choose a different cell.");
//             continue;
//         }
//         match board.has_someone_won() {
//             Some(someone) => {
//                 if board.current_player == someone {
//                     board.pretty_print();
//                     println!("Player {} has won!", board.current_player);
//                 } else if someone == BoardEntry::Blank {
//                     board.pretty_print();
//                     println!("It's a draw!");
//                 }
//                 break;
//             }
//             None => {}
//         }
//         // Switch player at end
//         board.change_player();
//     }
// }


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

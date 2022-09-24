use std::io;

enum BoardEntry {
  Blank,
  X,
  O
}

/*
 Board struct
*/
struct Board {
  board: Vec<Vec<BoardEntry>>
}

impl Board {
  fn get(&self, x: u8, y: u8) -> BoardEntry {
    return &(&self.board[x])[y];
  }
}

fn initialise_board() {
  let board = Board {
    board: vec![vec![BoardEntry::Blank; 3]; 3]
  };
}

/*
* Main tic tac toe board
*/
fn tictactoe_board() {

}
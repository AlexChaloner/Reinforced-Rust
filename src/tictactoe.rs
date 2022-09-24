use std::io;

#[derive(Clone)]
#[derive(Debug)]
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
  fn get(&self, x: usize, y: usize ) -> &BoardEntry {
    return &(&self.board[x])[y];
  }
  fn put(&mut self, x: usize, y: usize, entry: BoardEntry) {
    self.board[x][y] = entry;
  }
  // fn pretty_print(&self) {
  //   let mut string = "";
  //   for 
  // }
}

fn initialise_board() -> Board {
  return Board {
    board: vec![vec![BoardEntry::Blank; 3]; 3]
  };
}

/*
* Main tic tac toe board
*/
pub fn tictactoe_board() {
  let mut board = initialise_board();
  board.put(0, 0, BoardEntry::X);
  
}

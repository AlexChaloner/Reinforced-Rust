use core::fmt;

#[derive(Clone)]
#[derive(Debug)]
enum BoardEntry {
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
  fn pretty_print(&self) {
    let mut string = String::from("+---+---+---+\n");
    for y in 0..3 {
      string.push('|');
      for x in 0..3 {
        string.push_str(format!(" {} |", self.get(x, y)).as_str());
      }
      string.push_str("\n+---+---+---+\n");
    }
    println!("{}", string);
  }
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
  board.pretty_print();
}

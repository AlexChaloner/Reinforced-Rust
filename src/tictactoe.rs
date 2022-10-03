use core::fmt;
use std::io;
use rand::Rng;

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

/*
 Board struct
*/
#[derive(PartialEq, Eq, Hash)]
#[derive(Clone)]
pub struct Board(Vec<Vec<BoardEntry>>);

impl Board {
  pub fn get(&self, x: usize, y: usize ) -> BoardEntry {
    return self.0[x][y];
  }
  pub fn put(&mut self, x: usize, y: usize, entry: BoardEntry) {
    self.0[x][y] = entry;
  }
  pub fn pretty_print(&self) {
    let mut string = String::from("+---+---+---+\n");
    for x in 0..3 {
      string.push('|');
      for y in 0..3 {
        string.push_str(format!(" {} |", self.get(x, y)).as_str());
      }
      string.push_str("\n+---+---+---+\n");
    }
    println!("{}", string);
  }
}

pub fn create_initial_board() -> Board {
  return Board(vec![vec![BoardEntry::Blank; 3]; 3]);
}

pub fn has_someone_won(board: &Board) -> Option<BoardEntry> {
  // Check rows
  for y in 0..3 {
    let first = board.get(0, y);
    if first == BoardEntry::Blank { continue };
    if first == board.get(1, y) && first == board.get(2, y) {
      return Some(first);
    }
  }
  // Check columns
  for x in 0..3 {
    let first = board.get(x, 0);
    if first == BoardEntry::Blank { continue };
    if first == board.get(x, 1) && first == board.get(x, 2) {
      return Some(first);
    }
  }

  // Check down diagonal
  let first = board.get(0, 0);
  if first != BoardEntry::Blank && first == board.get(1, 1) && first == board.get(2, 2) {
    return Some(first);
  }

  // Check up diagonal
  let first = board.get(0, 2);
  if first != BoardEntry::Blank && first == board.get(1, 1) && first == board.get(2, 0) {
    return Some(first);
  }

  // Check if the board is filled
  for x in 0..=2 {
    for y in 0..=2 {
      if board.get(x, y) == BoardEntry::Blank {
        // Nobody has won yet
        return None;
      }
    }
  }

  // Indicates a draw
  return Some(BoardEntry::Blank);
}

fn get_move_input() -> Result<(usize, usize), ()> {
  let mut xy = String::new();
  io::stdin().read_line(&mut xy).expect("Failed to read line");
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
  if x <= 2 && y <= 2 {
    return Ok((x, y));
  } else {
    return Err(());
  }
}

pub fn two_player_tictactoe_game() {
  let mut board = create_initial_board();
  let mut player: BoardEntry;
  let who_starts = rand::thread_rng().gen_range(1..=2);
  if who_starts == 1 {
    player = BoardEntry::X;
  } else {
    player = BoardEntry::O;
  }
  loop {
    board.pretty_print();
    println!("Player {}, input your move: ", player);
    let (x, y) = match get_move_input() {
      Ok(moves) => moves,
      Err(_) => { continue },
    };
    if board.get(x, y) == BoardEntry::Blank {
      board.put(x, y, player);
    } else {
      println!("Cell is already filled, please choose a different cell.");
      continue;
    }
    match has_someone_won(&board) {
      Some(someone) => {
        if player == someone {
          board.pretty_print();
          println!("Player {} has won!", player);
        } else if someone == BoardEntry::Blank {
          board.pretty_print();
          println!("It's a draw!");
        }
        break;
      }
      None => {}
    }
    // Switch player at end
    match player {
      BoardEntry::X => player = BoardEntry::O,
      BoardEntry::O => player = BoardEntry::X,
      _ => panic!("Unknown Player"),
    }
  }
}

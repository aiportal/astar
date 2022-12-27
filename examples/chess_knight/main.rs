mod board;
use astar::Searcher;
use board::Board;

fn main() {
    let board = Board::new();

    let Some(path) = board.astar_search(&(0, 7), &(7, 0)) else { 
      println!("not found!" );
      return;
    };
    path.iter().enumerate().for_each(|(i, &(a, b))| {
      let ch = format!("{:x}", i).chars().next().unwrap();
      board.set(a, b, ch);
    });

    println!("{:?}", board);
    println!("path: {:?}, {}", path, path.len());
}

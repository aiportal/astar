mod board;
use astar::Searcher;
use board::Board;

fn main() {
    let grid = vec![vec!['.'; 9]; 9];
    let board = Board::new(grid);

    let Some(path) = board.astar_search(&(8, 0), &(0, 5)) else { 
        println!("path not found!" );
        return;
    };
    path.iter().for_each(|&(a, b)| board.set(a, b, '@'));

    println!("{:?}", board);
    println!("path: {:?}, {}", path, path.len());
}

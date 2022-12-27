use std::cell::RefCell;

pub struct Board {
    grid: RefCell<Vec<Vec<char>>>,
}

impl Board {
    pub fn new(grid: Vec<Vec<char>>) -> Board {
        Board {
            grid: RefCell::new(grid),
        }
    }
    pub fn get(&self, row: usize, col: usize) -> Option<char> {
        if let Some(row) = self.grid.borrow().get(row) {
            if let Some(ch) = row.get(col) {
                return Some(ch.clone());
            }
        }
        None
    }
    pub fn set(&self, row: usize, col: usize, value: char) {
        self.grid.borrow_mut()[row][col] = value;
    }
}

impl astar::Searcher for Board {
    type T = (usize, usize);
    type U = usize;
    type NeighborIter = std::vec::IntoIter<((usize, usize), usize)>;

    fn distance(&self, src: &Self::T, dst: &Self::T) -> Self::U {
        src.0.abs_diff(dst.0) * src.0.abs_diff(dst.0)
            + src.1.abs_diff(dst.1) * src.1.abs_diff(dst.1)
    }

    fn neighbors(&self, &(x, y): &Self::T) -> Self::NeighborIter {
        if self.get(x, y).is_none() {
            return vec![].into_iter(); // out of bounds
        }

        let coords = [
            (x.checked_sub(1), Some(y)),
            (Some(x), y.checked_sub(1)),
            (x.checked_add(1), Some(y)),
            (Some(x), y.checked_add(1)),
            (x.checked_sub(1), y.checked_sub(1)),
            (x.checked_add(1), y.checked_sub(1)),
            (x.checked_sub(1), y.checked_add(1)),
            (x.checked_add(1), y.checked_add(1)),
        ];

        let mut neighbors = Vec::with_capacity(4);
        for (x, y) in coords {
            let (Some(a), Some(b)) = (x, y) else { continue; };
            if self.get(a, b) == Some('.') {
                let item = ((a, b), 1);
                neighbors.push(item);
            }
        }

        neighbors.into_iter()
    }

    fn visited(&self, &(r, c): &Self::T, _costs: Self::U) {
        // let ch = format!("{:X}", costs).chars().next().unwrap();
        self.set(r, c, '*');
    }
}

use std::fmt::{Debug, Formatter, Result};

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let grid = self.grid.borrow();
        for row in grid.iter() {
            writeln!(f, "")?;
            for ch in row {
                write!(f, " {ch}")?;
            }
        }
        writeln!(f, "")
    }
}

use std::cell::RefCell;

pub struct Board {
    field: RefCell<[[char; 8]; 8]>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            field: RefCell::new([['.'; 8]; 8]),
        }
    }

    pub fn set(&self, row: usize, col: usize, value: char) {
        self.field.borrow_mut()[row][col] = value;
    }
}

impl astar::Searcher for Board {
    type T = (usize, usize);
    type U = usize;
    type NeighborIter = std::vec::IntoIter<((usize, usize), usize)>;

    fn distance(&self, src: &Self::T, dst: &Self::T) -> Self::U {
        src.0.abs_diff(dst.0) + src.1.abs_diff(dst.1)
    }

    fn neighbors(&self, &(x, y): &Self::T) -> Self::NeighborIter {
        if x >= 8 || y >= 8 {
            return vec![].into_iter(); // out of bounds
        }
        let coords = vec![
            (x.checked_sub(1), y.checked_sub(2)),
            (x.checked_sub(2), y.checked_sub(1)),
            (x.checked_sub(1), y.checked_add(2)),
            (x.checked_sub(2), y.checked_add(1)),
            (x.checked_add(1), y.checked_sub(2)),
            (x.checked_add(2), y.checked_sub(1)),
            (x.checked_add(1), y.checked_add(2)),
            (x.checked_add(2), y.checked_add(1)),
        ];
        coords
            .into_iter()
            .filter_map(|v| match v {
                (Some(a), Some(b)) if a < 8 && b < 8 => Some(((a, b), 3)),
                _ => None,
            })
            .collect::<Vec<_>>()
            .into_iter()
    }
}

use std::fmt::{Debug, Formatter, Result};

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let field = self.field.borrow();
        for row in field.iter() {
            writeln!(f, "")?;
            for ch in row {
                write!(f, " {ch}")?;
            }
        }
        writeln!(f, "")
    }
}

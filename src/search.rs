use crate::node::Node;
use core::hash::Hash;
use std::collections::{BinaryHeap, HashSet};
use std::ops::Add;
use std::rc::Rc;

pub trait Searcher {
    type T: Copy + Hash + PartialEq + Eq;
    type U: Copy + Ord + Default + Add<Output = Self::U>;

    /// Item = (key, costs)
    /// key: graph node key.
    /// costs: costs from cur to neighbor.
    type NeighborIter: Iterator<Item = (Self::T, Self::U)>;

    fn neighbors(&self, cur: &Self::T) -> Self::NeighborIter;
    fn distance(&self, src: &Self::T, dst: &Self::T) -> Self::U {
        Self::U::default()
    }
    fn visited(&self, item: &Self::T, costs: Self::U) {}

    fn astar_search(&self, src: &Self::T, dst: &Self::T) -> Option<Vec<Self::T>> {
        let start = Rc::new(Node {
            position: *src,
            costs: Self::U::default(),
            distance: self.distance(src, dst),
            parent: None,
        });

        let end = || -> Option<_> {
            let mut visited = HashSet::from([start.position]);
            let mut heap = BinaryHeap::from([start]);

            while let Some(node) = heap.pop() {
                for (neighbor, costs) in self.neighbors(&node.position) {
                    let next = Rc::new(Node {
                        position: neighbor,
                        costs: node.costs + costs,
                        distance: self.distance(&neighbor, dst),
                        parent: Some(Rc::clone(&node)),
                    });

                    if next.position == *dst {
                        return Some(next); // successed
                    }

                    if !visited.contains(&next.position) {
                        visited.insert(next.position);
                        self.visited(&next.position, next.costs);
                        heap.push(next);
                    }
                }
            }
            None
        }();

        let Some(node) = end else { return None};
        let path = trace_path(node);
        Some(path)
    }
}

fn trace_path<T: Copy, U>(node: Rc<Node<T, U>>) -> Vec<T> {
    let mut path = vec![node.position];
    let mut cur = &node;
    while cur.parent.is_some() {
        cur = cur.parent.as_ref().unwrap();
        path.push(cur.position);
    }
    path.reverse();
    path
}

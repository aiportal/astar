use std::cmp::Ordering;
use std::ops::Add;
use std::rc::Rc;

#[derive(Debug)]
pub struct Node<T, U> {
    pub position: T,
    pub costs: U,
    pub distance: U,
    pub parent: Option<Rc<Node<T, U>>>,
}

impl<T: PartialEq, U> PartialEq for Node<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}
impl<T: Eq, U> Eq for Node<T, U> {}

impl<T: Eq, U: PartialOrd> PartialOrd for Node<T, U>
where
    U: Copy + Add<Output = U>,
{
    fn partial_cmp(&self, other: &Node<T, U>) -> Option<Ordering> {
        (other.costs + other.distance).partial_cmp(&(self.costs + self.distance))
    }
}

impl<T: Eq, U: Ord> Ord for Node<T, U>
where
    U: Copy + Add<Output = U>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        (other.costs + other.distance).cmp(&(self.costs + self.distance))
    }
}

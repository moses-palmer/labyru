use std::collections::BinaryHeap;

use ::Pos;


/// A room position with a priority.
type PriorityPos = (isize, Pos);


/// A set of rooms and priorities.
///
/// This struct supports adding a position with a priority, retrieving the
/// position with the highest priority and querying whether a position is in the
/// set.
pub struct OpenSet {
    /// The heap containing prioritised positions.
    heap: BinaryHeap<PriorityPos>,
}


impl OpenSet {
    /// Creates a new open set.
    pub fn new() -> OpenSet {
        OpenSet { heap: BinaryHeap::new() }
    }

    /// Adds a position with a priority.
    ///
    /// # Arguments
    /// `priority` - The priority of the position.
    /// `pos` - The position.
    pub fn push(&mut self, priority: isize, pos: Pos) {
        self.heap.push((priority, pos));
    }

    /// Pops the room with the highest priority.
    pub fn pop(&mut self) -> Option<Pos> {
        match self.heap.pop() {
            Some((_, pos)) => Some(pos),
            None => None,
        }
    }

    /// Checks whether a position is in the set.
    ///
    /// # Arguments
    /// `pos` - The position.
    pub fn contains(&mut self, pos: Pos) -> bool {
        // TODO: Allow constant lookup time
        self.heap
            .iter()
            .filter(|&priority_pos| pos == priority_pos.1)
            .next()
            .is_some()
    }
}


#[cfg(test)]
mod tests {
    use open_set::OpenSet;

    #[test]
    fn pop_empty() {
        let mut os = OpenSet::new();

        assert!(os.pop().is_none());
    }


    #[test]
    fn pop_nonempty() {
        let mut os = OpenSet::new();

        os.push(0, (0, 0));
        assert!(os.pop().is_some());
    }


    #[test]
    fn pop_correct() {
        let mut os = OpenSet::new();
        let expected = (10, (1, 2));

        os.push(0, (3, 4));
        os.push(expected.0, expected.1);
        os.push(5, (5, 6));
        assert_eq!(os.pop(), Some(expected.1));
    }


    #[test]
    fn contains_same() {
        let mut os = OpenSet::new();
        let expected = (10, (1, 2));

        assert!(!os.contains(expected.1));
        os.push(0, (3, 4));
        assert!(!os.contains(expected.1));
        os.push(expected.0, expected.1);
        assert!(os.contains(expected.1));
        os.push(5, (5, 6));
        assert!(os.contains(expected.1));
        os.pop();
        assert!(!os.contains(expected.1));
    }
}

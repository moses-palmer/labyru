use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet};

use crate::matrix;

use crate::Maze;
use crate::WallPos;

/// The tuple `(current_wall, next_wall)`.
///
/// The second value can be used to determine whether the end has been reached;
/// it will be `None` for the last wall.
pub type FollowWallItem = (WallPos, Option<WallPos>);

impl<T> Maze<T>
where
    T: Clone,
{
    /// Walks from `from` to `to` along the shortest path.
    ///
    /// If the rooms are connected, the return value will iterate over the
    /// minimal set of rooms required to pass through to get from start to
    /// finish, including `from` and ` to`.
    ///
    /// # Example
    ///
    /// ```
    /// # use maze::matrix;
    /// # use maze::walk::*;
    /// # let maze = maze::Shape::Hex.create::<u32>(5, 5)
    /// #     .initialize(
    /// #         maze::initialize::Method::Winding,
    /// #         &mut maze::initialize::LFSR::new(12345),
    /// #     );
    ///
    /// for (i, pos) in maze
    ///     .walk(
    ///         matrix::Pos { col: 0, row: 0 },
    ///         matrix::Pos {
    ///             col: maze.width() as isize - 1,
    ///             row: maze.height() as isize - 1,
    ///         },
    ///     )
    ///     .unwrap()
    ///     .into_iter()
    ///     .enumerate()
    /// {
    ///     println!("{:?} is room #{} on the path", pos, i);
    /// }
    ///
    /// ```
    ///
    /// # Arguments
    /// *  `from` - The starting position.
    /// *  `to` - The desired goal.
    pub fn walk(&self, from: matrix::Pos, to: matrix::Pos) -> Option<Path<T>> {
        // Reverse the positions to return the rooms in correct order
        let (start, end) = (to, from);

        // The heuristic for a room position
        let h = |pos: matrix::Pos| {
            let dx = (pos.col - end.col).abs();
            let dy = (pos.row - end.row).abs();
            dx * dx + dy * dy
        };

        // The room positions already evaluated
        let mut closed_set = HashSet::new();

        // The room positions pending evaluation and their cost
        let mut open_set = OpenSet::new();
        open_set.push(std::isize::MAX, start);

        // The cost from start to a room along the best known path
        let mut g_score = HashMap::new();
        g_score.insert(start, 0isize);

        // The estimated cost from start to end through a room
        let mut f_score = HashMap::new();
        f_score.insert(start, h(start));

        // The room from which we entered a room; when we reach the end, we use
        // this to backtrack to the start
        let mut came_from = BTreeMap::new();

        while let Some(current) = open_set.pop() {
            // Have we reached the target?
            if current == end {
                return Some(Path::new(self, current, came_from));
            }

            closed_set.insert(current);
            for wall in self.doors(current) {
                // Find the next room, and continue if we have already evaluated
                // it to a better distance, or it is outside of the maze
                let (next, _) = self.back((current, wall));
                if !self.is_inside(next)
                    || (closed_set.contains(&next)
                        && g_score[&next] <= g_score[&current] + 1)
                {
                    continue;
                }

                // The cost to get to this room is one more that the room from
                // which we came
                let g = g_score[&current] + 1;
                let f = g + h(next);

                if !open_set.contains(current) || g < g_score[&current] {
                    came_from.insert(next, current);
                    g_score.insert(next, g);
                    f_score.insert(next, f);

                    if !open_set.contains(current) {
                        open_set.push(f, next);
                    }
                }
            }
        }

        None
    }

    /// Follows a wall.
    ///
    /// This method will follow a wall without passing through any walls. When
    /// the starting wall is encountered, no more walls will be returned.
    ///
    /// # Arguments
    /// *  `wall_pos` - The starting wall position.
    pub fn follow_wall<'a>(
        &'a self,
        wall_pos: WallPos,
    ) -> impl Iterator<Item = FollowWallItem> + 'a {
        Follower::new(self, wall_pos)
    }
}

/// A path through a maze.
///
/// This struct describes the path through a maze by maintaining a mapping from
/// a room position to the next room.
pub struct Path<'a, T>
where
    T: Clone,
{
    /// The maze being walked.
    pub(crate) maze: &'a Maze<T>,

    /// The starting position.
    start: matrix::Pos,

    /// The backing map.
    map: BTreeMap<matrix::Pos, matrix::Pos>,
}

impl<'a, T> Path<'a, T>
where
    T: Clone,
{
    /// Stores the path from a starting position and a supporting map.
    ///
    /// It is possible to walk indefinitely if the mapping contains circular
    /// references.
    pub fn new(
        maze: &'a Maze<T>,
        start: matrix::Pos,
        map: BTreeMap<matrix::Pos, matrix::Pos>,
    ) -> Self {
        Path { maze, start, map }
    }
}

impl<'a, T> IntoIterator for &'a Path<'a, T>
where
    T: Clone,
{
    type Item = matrix::Pos;
    type IntoIter = Walker<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Walker {
            path: self,
            current: self.start,
            increment: false,
        }
    }
}

pub struct Walker<'a, T>
where
    T: Clone,
{
    /// The actual path to walk.
    path: &'a Path<'a, T>,

    /// The current position.
    current: matrix::Pos,

    /// Whether `next` should return the next element. This will be false only
    /// for the first call to `next`.
    increment: bool,
}

impl<'a, T> Iterator for Walker<'a, T>
where
    T: Clone,
{
    type Item = matrix::Pos;

    /// Yields the next room position.
    fn next(&mut self) -> Option<matrix::Pos> {
        if self.increment {
            match self.path.map.get(&self.current) {
                Some(next) => {
                    self.current = *next;
                    Some(*next)
                }
                None => None,
            }
        } else {
            self.increment = true;
            Some(self.current)
        }
    }
}

/// Follows a wall.
struct Follower<'a, T>
where
    T: Clone,
{
    /// The maze.
    maze: &'a Maze<T>,

    /// The starting position.
    start_pos: WallPos,

    /// The current position.
    current: WallPos,

    /// Whether we have finished following walls.
    finished: bool,
}

impl<'a, T> Follower<'a, T>
where
    T: Clone,
{
    pub(self) fn new(maze: &'a Maze<T>, start_pos: WallPos) -> Self {
        Self {
            maze,
            start_pos,
            current: start_pos,
            finished: false,
        }
    }

    /// Retrieves the next wall position.
    ///
    /// The next wall position will be reachable from `wall_pos` without passing
    /// through any walls, and it will share a corner. Repeatedly calling this
    /// method will yield walls clockwise inside a cavity in the maze.
    ///
    /// # Arguments
    /// *  `wall_pos`- The wall position for which to retrieve a room.
    fn next_wall_pos(&self, wall_pos: WallPos) -> WallPos {
        self.maze
            .corner_walls((wall_pos.0, wall_pos.1.next))
            .find(|&next| !self.maze.is_open(next))
            .unwrap_or_else(|| self.maze.back(wall_pos))
    }
}

impl<'a, T> Iterator for Follower<'a, T>
where
    T: Clone,
{
    type Item = FollowWallItem;

    /// Iterates over all wall positions.
    ///
    /// Wall positions are returned in the pair _(from, to)_. The last iteration
    /// before this iterator is exhausted will return _to_ as `None`.
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            None
        } else {
            let previous = self.current;
            self.current = self.next_wall_pos(self.current);
            self.finished = self.current == self.start_pos;
            Some((
                previous,
                if self.finished {
                    None
                } else {
                    Some(self.current)
                },
            ))
        }
    }
}

/// A room position with a priority.
type PriorityPos = (isize, matrix::Pos);

/// A set of rooms and priorities.
///
/// This struct supports adding a position with a priority, retrieving the
/// position with the highest priority and querying whether a position is in the
/// set.
struct OpenSet {
    /// The heap containing prioritised positions.
    heap: BinaryHeap<PriorityPos>,
}

impl OpenSet {
    /// Creates a new open set.
    pub fn new() -> OpenSet {
        OpenSet {
            heap: BinaryHeap::new(),
        }
    }

    /// Adds a position with a priority.
    ///
    /// # Arguments
    /// *  priority` - The priority of the position.
    /// *  pos` - The position.
    pub fn push(&mut self, priority: isize, pos: matrix::Pos) {
        self.heap.push((priority, pos));
    }

    /// Pops the room with the highest priority.
    pub fn pop(&mut self) -> Option<matrix::Pos> {
        match self.heap.pop() {
            Some((_, pos)) => Some(pos),
            None => None,
        }
    }

    /// Checks whether a position is in the set.
    ///
    /// # Arguments
    /// *  `pos` - The position.
    pub fn contains(&mut self, pos: matrix::Pos) -> bool {
        // TODO: Allow constant lookup time
        self.heap.iter().any(|&priority_pos| pos == priority_pos.1)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use maze_test::maze_test;

    use super::*;
    use crate::test_utils::*;

    #[maze_test]
    fn walk_empty(maze: TestMaze) {
        let map = BTreeMap::new();

        assert_eq!(
            Path::new(&maze, matrix_pos(0, 0), map)
                .into_iter()
                .collect::<Vec<matrix::Pos>>(),
            vec![matrix_pos(0, 0)]
        );
    }

    #[maze_test]
    fn walk_from_unknown(maze: TestMaze) {
        let mut map = BTreeMap::new();
        map.insert(matrix_pos(1, 1), matrix_pos(2, 2));

        assert_eq!(
            Path::new(&maze, matrix_pos(0, 0), map)
                .into_iter()
                .collect::<Vec<matrix::Pos>>(),
            vec![matrix_pos(0, 0)]
        );
    }

    #[maze_test]
    fn walk_path(maze: TestMaze) {
        let mut map = BTreeMap::new();
        map.insert(matrix_pos(1, 1), matrix_pos(2, 2));
        map.insert(matrix_pos(2, 2), matrix_pos(2, 3));
        map.insert(matrix_pos(2, 3), matrix_pos(2, 4));

        assert_eq!(
            Path::new(&maze, matrix_pos(1, 1), map)
                .into_iter()
                .collect::<Vec<matrix::Pos>>(),
            vec![
                matrix_pos(1, 1),
                matrix_pos(2, 2),
                matrix_pos(2, 3),
                matrix_pos(2, 4)
            ]
        );
    }

    #[maze_test]
    fn walk_disconnected(maze: TestMaze) {
        assert!(maze.walk(matrix_pos(0, 0), matrix_pos(0, 1)).is_none());
    }

    #[maze_test]
    fn walk_same(maze: TestMaze) {
        let from = matrix_pos(0, 0);
        let to = matrix_pos(0, 0);
        let expected = vec![matrix_pos(0, 0)];
        assert!(
            maze.walk(from, to)
                .unwrap()
                .into_iter()
                .collect::<Vec<matrix::Pos>>()
                == expected
        );
    }

    #[maze_test]
    fn walk_simple(mut maze: TestMaze) {
        let log = Navigator::new(&mut maze).down(true).stop();

        let from = log.first().unwrap();
        let to = log.last().unwrap();
        let expected = vec![*from, *to];
        assert!(
            maze.walk(*from, *to)
                .unwrap()
                .into_iter()
                .collect::<Vec<matrix::Pos>>()
                == expected
        );
    }

    #[maze_test]
    fn walk_shortest(mut maze: TestMaze) {
        let log = Navigator::new(&mut maze)
            .down(true)
            .right(true)
            .right(true)
            .up(true)
            .stop();

        let from = log.first().unwrap();
        let to = log.last().unwrap();
        assert!(
            maze.walk(*from, *to)
                .unwrap()
                .into_iter()
                .collect::<Vec<matrix::Pos>>()
                .len()
                <= log.len()
        );
    }

    #[test]
    fn pop_empty() {
        let mut os = OpenSet::new();

        assert!(os.pop().is_none());
    }

    #[test]
    fn pop_nonempty() {
        let mut os = OpenSet::new();

        os.push(0, matrix_pos(0, 0));
        assert!(os.pop().is_some());
    }

    #[test]
    fn pop_correct() {
        let mut os = OpenSet::new();
        let expected = (10, matrix_pos(1, 2));

        os.push(0, matrix_pos(3, 4));
        os.push(expected.0, expected.1);
        os.push(5, matrix_pos(5, 6));
        assert_eq!(os.pop(), Some(expected.1));
    }

    #[test]
    fn contains_same() {
        let mut os = OpenSet::new();
        let expected = (10, matrix_pos(1, 2));

        assert!(!os.contains(expected.1));
        os.push(0, matrix_pos(3, 4));
        assert!(!os.contains(expected.1));
        os.push(expected.0, expected.1);
        assert!(os.contains(expected.1));
        os.push(5, matrix_pos(5, 6));
        assert!(os.contains(expected.1));
        os.pop();
        assert!(!os.contains(expected.1));
    }
}

use std::collections::BinaryHeap;

use bit_set::BitSet;

use crate::matrix;

use crate::matrix::Matrix;
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
            (dx * dx + dy * dy) as u32
        };

        // The room positions pending evaluation and their cost
        let mut open_set = OpenSet::new(self.width(), self.height());
        open_set.push(std::u32::MAX, start);

        let mut rooms = Matrix::<Room>::new(self.width(), self.height());
        rooms[start].g = 0;
        rooms[start].f = h(start);

        while let Some(current) = open_set.pop() {
            // Have we reached the target?
            if current == end {
                return Some(Path::new(self, start, end, rooms));
            }

            rooms[current].visited = true;
            for wall in self.doors(current) {
                // Find the next room, and continue if we have already evaluated
                // it to a better distance, or it is outside of the maze
                let (next, _) = self.back((current, wall));
                if !self.is_inside(next)
                    || (rooms[next].visited
                        && rooms[next].g <= rooms[current].g + 1)
                {
                    continue;
                }

                // The cost to get to this room is one more that the room from
                // which we came
                let g = rooms[current].g + 1;
                let f = g + h(next);

                let current_in_open_set = open_set.contains(current);
                if !current_in_open_set || g < rooms[current].g {
                    rooms[next].g = g;
                    rooms[next].f = f;
                    rooms[next].came_from = Some(current);

                    if !current_in_open_set {
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
    /// The direction of walking along a wall is from the point where its span
    /// starts to where it ends.
    ///
    /// If the starting position is an open wall, the iterator will contain no
    /// elements.
    ///
    /// # Arguments
    /// *  `wall_pos` - The starting wall position.
    pub fn follow_wall(
        &self,
        wall_pos: WallPos,
    ) -> impl Iterator<Item = FollowWallItem> + '_ {
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

    /// The backing room matrix.
    rooms: matrix::Matrix<Room>,

    /// The start position.
    a: matrix::Pos,

    /// The end position.
    b: matrix::Pos,
}

impl<'a, T> Path<'a, T>
where
    T: Clone,
{
    /// Stores a path in a maze.
    ///
    /// # Arguments
    /// *  `maze` - The maze being walked.
    /// *  `start` - The start position.
    /// *  `rooms` - The backing room matrix.
    pub(self) fn new(
        maze: &'a Maze<T>,
        start: matrix::Pos,
        end: matrix::Pos,
        rooms: matrix::Matrix<Room>,
    ) -> Self {
        Path {
            maze,
            rooms,
            a: end,
            b: start,
        }
    }
}

impl<'a, T> IntoIterator for &'a Path<'a, T>
where
    T: Clone,
{
    type Item = matrix::Pos;
    type IntoIter = <Vec<matrix::Pos> as IntoIterator>::IntoIter;

    /// Backtraces a path by following the `came_from` fields.
    ///
    /// To generate
    ///
    /// # Arguments
    /// *  `start` - The starting position.
    /// *  `end` - The end position.
    ///
    /// # Panics
    /// If the backing room matrix is incomplete.
    fn into_iter(self) -> Self::IntoIter {
        let (a, b) = (self.a, self.b);
        let mut result = Vec::with_capacity(self.rooms[a].f as usize);
        result.push(a);

        let mut current = a;
        while current != b {
            if let Some(next) = self.rooms[current].came_from {
                result.push(next);
                if current == b {
                    break;
                } else {
                    current = next;
                }
            } else {
                panic!("attempted to backtrace an incomplete path!");
            }
        }

        result.into_iter()
    }
}

/// A rooms description for the walk algorithm.
#[derive(Clone, Debug)]
struct Room {
    /// The F score.
    ///
    ///This is the cost from start to a room along the best known path
    f: u32,

    /// The G score.
    ///
    /// This is the estimated cost from start to end through a room.
    g: u32,

    /// Whether the rooms has been visited.
    visited: bool,

    /// The room from which we came.
    ///
    /// When the algorithm has competed, this will be the room on the shortest
    /// path.
    came_from: Option<matrix::Pos>,
}

impl Default for Room {
    fn default() -> Self {
        Room {
            f: u32::MAX,
            g: u32::MAX,
            visited: false,
            came_from: None,
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
            finished: maze.is_open(start_pos),
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
            .corner_walls_start((wall_pos.0, wall_pos.1.next))
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
type PriorityPos = (u32, matrix::Pos);

/// A set of rooms and priorities.
///
/// This struct supports adding a position with a priority, retrieving the
/// position with the highest priority and querying whether a position is in the
/// set.
struct OpenSet {
    /// The width of the set.
    width: usize,

    /// The height of the set.
    height: usize,

    /// The heap containing prioritised positions.
    heap: BinaryHeap<PriorityPos>,

    /// The positions present in the heap.
    present: BitSet,
}

impl OpenSet {
    /// Creates a new open set.
    pub fn new(width: usize, height: usize) -> OpenSet {
        OpenSet {
            width,
            height,
            heap: BinaryHeap::new(),
            present: BitSet::with_capacity(width * height),
        }
    }

    /// Adds a position with a priority.
    ///
    /// # Arguments
    /// *  priority` - The priority of the position.
    /// *  pos` - The position.
    pub fn push(&mut self, priority: u32, pos: matrix::Pos) {
        if let Some(index) = self.index(pos) {
            self.heap.push((priority, pos));
            self.present.insert(index);
        }
    }

    /// Pops the room with the highest priority.
    pub fn pop(&mut self) -> Option<matrix::Pos> {
        if let Some(pos) = self.heap.pop().map(|(_, pos)| pos) {
            if let Some(index) = self.index(pos) {
                self.present.remove(index);
            }
            Some(pos)
        } else {
            None
        }
    }

    /// Checks whether a position is in the set.
    ///
    /// # Arguments
    /// *  `pos` - The position.
    pub fn contains(&mut self, pos: matrix::Pos) -> bool {
        self.index(pos)
            .map(|i| self.present.contains(i))
            .unwrap_or(false)
    }

    /// Calculates the index of a position.
    ///
    /// If the position is outside of this set, nothing is returned.
    ///
    /// # Arguments
    /// *  `pos` - The position.
    fn index(&self, pos: matrix::Pos) -> Option<usize> {
        if pos.col >= 0
            && pos.row >= 0
            && pos.col < self.width as isize
            && pos.row < self.height as isize
        {
            Some(pos.col as usize + pos.row as usize * self.width)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use maze_test::maze_test;

    use super::*;
    use crate::test_utils::*;

    #[maze_test]
    fn walk_single(maze: TestMaze) {
        let map = Matrix::<Room>::new_with_data(10, 10, |_| Room {
            f: 0,
            ..Default::default()
        });

        assert_eq!(
            Path::new(&maze, matrix_pos(0, 0), matrix_pos(0, 0), map)
                .into_iter()
                .collect::<Vec<matrix::Pos>>(),
            vec![matrix_pos(0, 0)]
        );
    }

    #[maze_test]
    fn walk_path(maze: TestMaze) {
        let mut map = Matrix::<Room>::new_with_data(10, 10, |_| Room {
            f: 0,
            ..Default::default()
        });
        map[matrix_pos(1, 1)].came_from = Some(matrix_pos(2, 2));
        map[matrix_pos(2, 2)].came_from = Some(matrix_pos(2, 3));
        map[matrix_pos(2, 3)].came_from = Some(matrix_pos(2, 4));

        assert_eq!(
            Path::new(&maze, matrix_pos(2, 4), matrix_pos(1, 1), map)
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
        assert_eq!(
            maze.walk(from, to)
                .unwrap()
                .into_iter()
                .collect::<Vec<matrix::Pos>>(),
            expected,
        );
    }

    #[maze_test]
    fn walk_simple(mut maze: TestMaze) {
        let log = Navigator::new(&mut maze).down(true).stop();

        let from = log.first().unwrap();
        let to = log.last().unwrap();
        let expected = vec![*from, *to];
        assert_eq!(
            maze.walk(*from, *to)
                .unwrap()
                .into_iter()
                .collect::<Vec<matrix::Pos>>(),
            expected,
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

    #[maze_test]
    fn follow_wall_order(maze: TestMaze) {
        let start =
            maze.wall_positions((0isize, 0isize).into()).next().unwrap();

        for (a, b) in maze.follow_wall(start) {
            if let Some(b) = b {
                assert!(is_close(
                    maze.center(a.0) + a.1.span.1,
                    maze.center(b.0) + b.1.span.0,
                ));
            }
        }
    }

    #[test]
    fn pop_empty() {
        let mut os = OpenSet::new(10, 10);

        assert!(os.pop().is_none());
    }

    #[test]
    fn pop_nonempty() {
        let mut os = OpenSet::new(10, 10);

        os.push(0, matrix_pos(0, 0));
        assert!(os.pop().is_some());
    }

    #[test]
    fn pop_correct() {
        let mut os = OpenSet::new(10, 10);
        let expected = (10, matrix_pos(1, 2));

        os.push(0, matrix_pos(3, 4));
        os.push(expected.0, expected.1);
        os.push(5, matrix_pos(5, 6));
        assert_eq!(os.pop(), Some(expected.1));
    }

    #[test]
    fn contains_same() {
        let mut os = OpenSet::new(10, 10);
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

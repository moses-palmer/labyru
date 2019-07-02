use std;

use crate::matrix;
use crate::util::open_set;
use crate::wall;

use crate::Maze;
use crate::WallPos;

impl Maze {
    /// Walks from `from` to `to` along the shortest path.
    ///
    /// If the rooms are connected, the return value will iterate over the
    /// minimal set of rooms required to pass through to get from start to
    /// finish, including `from` and ` to`.
    ///
    /// # Arguments
    /// * `from` - The starting position.
    /// * `to` - The desired goal.
    pub fn walk(&self, from: matrix::Pos, to: matrix::Pos) -> Option<Path> {
        // Reverse the positions to return the rooms in correct order
        let (start, end) = (to, from);

        // The heuristic for a room position
        let h = |pos: matrix::Pos| {
            (pos.col - end.col).abs() + (pos.row - end.row).abs()
        };

        // The room positions already evaluated
        let mut closed_set = std::collections::HashSet::new();

        // The room positions pending evaluation and their cost
        let mut open_set = open_set::OpenSet::new();
        open_set.push(std::isize::MAX, start);

        // The cost from start to a room along the best known path
        let mut g_score = std::collections::HashMap::new();
        g_score.insert(start, 0isize);

        // The estimated cost from start to end through a room
        let mut f_score = std::collections::HashMap::new();
        f_score.insert(start, h(start));

        // The room from which we entered a room; when we reach the end, we use
        // this to backtrack to the start
        let mut came_from = std::collections::HashMap::new();

        while let Some(current) = open_set.pop() {
            // Have we reached the target?
            if current == end {
                return Some(Path::new(self, current, came_from));
            }

            closed_set.insert(current);
            for wall in self.walls(current) {
                // Ignore closed walls
                if !self.is_open((current, wall)) {
                    continue;
                }

                // Find the next room, and continue if we have already evaluated
                // it, or it is outside of the maze
                let (next, _) = self.back((current, wall));
                if !self.rooms.is_inside(next) || closed_set.contains(&next) {
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
    /// * `wall_pos` - The starting wall position.
    pub fn follow_wall<'a>(
        &'a self,
        wall_pos: WallPos,
    ) -> impl Iterator<Item = (WallPos, Option<WallPos>)> + 'a {
        Follower::new(self, wall_pos)
    }
}

/// A path through a maze.
///
/// This struct describes the path through a maze by maintaining a mapping from
/// a room position to the next room.
pub struct Path<'a> {
    /// The maze being walked.
    pub(crate) maze: &'a Maze,

    /// The starting position.
    start: matrix::Pos,

    /// The backing map.
    map: std::collections::HashMap<matrix::Pos, matrix::Pos>,
}

impl<'a> Path<'a> {
    /// Stores the path from a starting position and a supporting map.
    ///
    /// It is possible to walk indefinitely if the mapping contains circular
    /// references.
    pub fn new(
        maze: &'a Maze,
        start: matrix::Pos,
        map: std::collections::HashMap<matrix::Pos, matrix::Pos>,
    ) -> Self {
        Path { maze, start, map }
    }
}

impl<'a> IntoIterator for &'a Path<'a> {
    type Item = matrix::Pos;
    type IntoIter = Walker<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Walker {
            path: self,
            current: self.start,
            increment: false,
        }
    }
}

pub struct Walker<'a> {
    /// The actual path to walk.
    path: &'a Path<'a>,

    /// The current position.
    current: matrix::Pos,

    /// Whether `next` should return the next element. This will be false only
    /// for the first call to `next`.
    increment: bool,
}

impl<'a> Iterator for Walker<'a> {
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
struct Follower<'a> {
    /// The maze.
    maze: &'a Maze,

    /// The starting position.
    start_pos: WallPos,

    /// The current position.
    current: WallPos,

    /// Whether we have finished following walls.
    finished: bool,
}

impl<'a> Follower<'a> {
    pub(self) fn new(maze: &'a Maze, start_pos: WallPos) -> Self {
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
    /// * `wall_pos`- The wall position for which to retrieve a room.
    fn next_wall_pos(&self, wall_pos: WallPos) -> WallPos {
        let all = self.maze.all_walls();
        let back = self.maze.back(wall_pos);
        let matrix::Pos { col, row } = back.0;
        all[back.1.index]
            .corner_wall_offsets
            .iter()
            // Convert the offsets to wall positions
            .map(|&wall::Offset { dx, dy, wall }| {
                (
                    matrix::Pos {
                        col: col + dx,
                        row: row + dy,
                    },
                    all[wall],
                )
            })
            // Find the first closed wall
            .skip_while(|&next| self.maze.is_open(next))
            // Yield the first wall we encounter, or the back of the original
            // wall if we encounter no other wall
            .next()
            .unwrap_or(back)
    }
}

impl<'a> Iterator for Follower<'a> {
    type Item = (WallPos, Option<WallPos>);

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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use maze_test::maze_test;

    use super::*;
    use crate::test_utils::*;
    use crate::*;

    #[maze_test]
    fn walk_empty(maze: Maze) {
        let map = HashMap::new();

        assert_eq!(
            Path::new(&maze, matrix_pos(0, 0), map)
                .into_iter()
                .collect::<Vec<matrix::Pos>>(),
            vec![matrix_pos(0, 0)]
        );
    }

    #[maze_test]
    fn walk_from_unknown(maze: Maze) {
        let mut map = HashMap::new();
        map.insert(matrix_pos(1, 1), matrix_pos(2, 2));

        assert_eq!(
            Path::new(&maze, matrix_pos(0, 0), map)
                .into_iter()
                .collect::<Vec<matrix::Pos>>(),
            vec![matrix_pos(0, 0)]
        );
    }

    #[maze_test]
    fn walk_path(maze: Maze) {
        let mut map = HashMap::new();
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
    fn walk_disconnected(maze: Maze) {
        assert!(maze.walk(matrix_pos(0, 0), matrix_pos(0, 1)).is_none());
    }

    #[maze_test]
    fn walk_same(maze: Maze) {
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
    fn walk_simple(mut maze: Maze) {
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
    fn walk_shortest(mut maze: Maze) {
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
}

use std;

use matrix;
use util::open_set;

use Maze;
use WallPos;


/// A container that supports walking.
pub trait Walkable {
    /// Walks from `from` to `to` along the sortest path.
    ///
    /// If the rooms are connected, the return value will iterate over the
    /// minimal set of rooms required to pass through to get from start to
    /// finish, including `from` and ` to`.
    ///
    /// # Arguments
    /// * `from` - The starting position.
    /// * `to` - The desired goal.
    fn walk(&self, from: matrix::Pos, to: matrix::Pos) -> Option<Walker>;

    /// Follows a wall.
    ///
    /// This method will follow a wall without passing through any walls. When
    /// the starting wall is encountered, no more walls will be returned.
    ///
    /// # Arguments
    /// * `wall_pos` - The starting wall position.
    fn follow_wall(&self, wall_pos: WallPos) -> Follower;
}


impl<'a, M> Walkable for M
where
    M: Maze + 'a,
{
    fn walk(&self, from: matrix::Pos, to: matrix::Pos) -> Option<Walker> {
        // Reverse the positions to return the rooms in correct order
        let (start, end) = (to, from);

        /// The heuristic for a room position
        let h =
            |pos: matrix::Pos| (pos.0 - end.0).abs() + (pos.1 - end.1).abs();

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
                return Some(Walker::new(current, came_from));
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
                if !self.rooms().is_inside(next) || closed_set.contains(&next) {
                    continue;
                }

                // The cost to get to this room is one more that the room from
                // which we came
                let g = g_score.get(&current).unwrap() + 1;
                let f = g + h(next);

                if !open_set.contains(current) ||
                    g < *g_score.get(&current).unwrap()
                {
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

    fn follow_wall(&self, wall_pos: WallPos) -> Follower {
        Follower::new(self, wall_pos)
    }
}


/// A maze walker.
///
/// This struct supports walking through a map. From a starting position, it
/// will yield all room positions by mapping a position to the next.
///
/// It will continue until a position maps to `None`. All positions encountered,
/// including `start` and the position yielding `None`, will be returned.
pub struct Walker {
    /// The current position.
    current: matrix::Pos,

    /// Whether `next` should return the next element. This will be true only
    /// for the first call to `next`.
    increment: bool,

    /// The backing map.
    map: std::collections::HashMap<matrix::Pos, matrix::Pos>,
}


impl Walker {
    /// Creates a walker from a starting position and a supporting map.
    ///
    /// It is possible to walk indefinitely if the mapping contains circular
    /// references.
    pub fn new(
        start: matrix::Pos,
        map: std::collections::HashMap<matrix::Pos, matrix::Pos>,
    ) -> Walker {
        Walker {
            current: start,
            increment: false,
            map: map,
        }
    }
}


impl Iterator for Walker {
    type Item = matrix::Pos;

    /// Yields the next room position.
    fn next(&mut self) -> Option<matrix::Pos> {
        if self.increment {
            match self.map.get(&self.current) {
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
pub struct Follower<'a> {
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
    pub fn new(maze: &'a Maze, start_pos: WallPos) -> Self {
        Self {
            maze: maze,
            start_pos: start_pos,
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
        let (x, y) = back.0;
        all[back.1.index].corner_wall_offsets
            .into_iter()

            // Convert the offsets to wall positions
            .map(|&((dx, dy), wall_index)| ((x + dx, y + dy), all[wall_index]))

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

    use test_utils::*;
    use ::*;
    use super::*;

    #[test]
    fn walk_empty() {
        let map = HashMap::new();

        assert_eq!(
            Walker::new((0, 0), map).collect::<Vec<matrix::Pos>>(),
            vec![(0, 0)]
        );
    }


    #[test]
    fn walk_from_unknown() {
        let mut map = HashMap::new();
        map.insert((1, 1), (2, 2));

        assert_eq!(
            Walker::new((0, 0), map).collect::<Vec<matrix::Pos>>(),
            vec![(0, 0)]
        );
    }


    #[test]
    fn walk_path() {
        let mut map = HashMap::new();
        map.insert((1, 1), (2, 2));
        map.insert((2, 2), (2, 3));
        map.insert((2, 3), (2, 4));

        assert_eq!(
            Walker::new((1, 1), map).collect::<Vec<matrix::Pos>>(),
            vec![(1, 1), (2, 2), (2, 3), (2, 4)]
        );
    }

    maze_test!(walk_disconnected, fn test(maze: &mut Maze) {
        assert!(maze.walk((0, 0), (0, 1)).is_none());
    });


    maze_test!(walk_same, fn test(maze: &mut Maze) {
        let from = (0, 0);
        let to = (0, 0);
        let expected = vec![(0, 0)];
        assert!(
            maze.walk(from, to)
                .unwrap()
                .collect::<Vec<matrix::Pos>>() == expected
        );
    });


    maze_test!(walk_simple, fn test(maze: &mut Maze) {
        let log = Navigator::new(maze)
            .down(true)
            .stop();

        let from = log.first().unwrap();
        let to = log.last().unwrap();
        let expected = vec![*from, *to];
        assert!(
            maze.walk(*from, *to)
                .unwrap()
                .collect::<Vec<matrix::Pos>>() == expected
        );
    });


    maze_test!(walk_shortest, fn test(maze: &mut Maze) {
        let log = Navigator::new(maze)
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
                .collect::<Vec<matrix::Pos>>().len() <= log.len()
        );
    });
}
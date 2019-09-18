#![cfg_attr(feature = "cargo-clippy", deny(clippy::all))]

use serde::{Deserialize, Serialize};

#[cfg(test)]
#[macro_use]
mod test_utils;

#[macro_use]
pub mod wall;

#[macro_use]
pub mod shape;
pub use self::shape::Shape;

pub mod initialize;
pub mod matrix;
pub mod physical;
pub mod render;
pub mod room;
pub mod walk;

mod util;

/// A wall of a room.
pub type WallPos = (matrix::Pos, &'static wall::Wall);

/// A maze contains rooms and has methods for managing paths and doors.
#[derive(Clone, Deserialize, Serialize)]
pub struct Maze {
    /// The shape of the rooms.
    shape: Shape,

    /// The actual rooms.
    rooms: room::Rooms,
}

impl Maze {
    /// Creates an uninitialised maze.
    ///
    /// # Arguments
    /// *  `shape` - The shape of the rooms.
    /// *  `width` - The width, in rooms, of the maze.
    /// *  `height` - The height, in rooms, of the maze.
    pub fn new(shape: Shape, width: usize, height: usize) -> Self {
        let rooms = room::Rooms::new(width, height);
        Self { shape, rooms }
    }

    /// Returns the width of the maze.
    pub fn width(&self) -> usize {
        self.rooms.width
    }

    /// Returns the height of the maze.
    pub fn height(&self) -> usize {
        self.rooms.height
    }

    /// Returns the shape of the maze.
    pub fn shape(&self) -> Shape {
        self.shape
    }

    /// Determines whether a position is inside of the maze.
    ///
    /// # Arguments
    /// *  `pos` - The romm position.
    pub fn is_inside(&self, pos: matrix::Pos) -> bool {
        self.rooms.is_inside(pos)
    }

    /// Returns whether a specified wall is open.
    ///
    /// # Arguments
    /// *  `wall_pos` - The wall position.
    pub fn is_open(&self, wall_pos: WallPos) -> bool {
        match self.rooms.get(wall_pos.0) {
            Some(room) => room.is_open(wall_pos.1),
            None => false,
        }
    }

    /// Finds the wall connecting two rooms, and if it exists, returns it.
    ///
    /// # Arguments
    /// *  `pos1` - The first room position. The returned wall position will be
    ///    in this room.
    /// *  `pos2` - The second room position.
    pub fn connecting_wall(
        &self,
        pos1: matrix::Pos,
        pos2: matrix::Pos,
    ) -> Option<WallPos> {
        self.walls(pos1)
            .iter()
            .find(|wall| {
                (pos1.col + wall.dir.0 == pos2.col)
                    && (pos1.row + wall.dir.1 == pos2.row)
            })
            .map(|&wall| (pos1, wall))
    }

    /// Returns whether two rooms are connected.
    ///
    /// Two rooms are connected if there is an open wall between them, or if
    /// they are the same room.
    ///
    /// # Arguments
    /// *  `pos1` - The first room.
    /// *  `pos2` - The second room.
    pub fn connected(&self, pos1: matrix::Pos, pos2: matrix::Pos) -> bool {
        if pos1 == pos2 {
            true
        } else if let Some(wall) = self.walls(pos1).iter().find(|wall| {
            (pos1.col + wall.dir.0 == pos2.col)
                && (pos1.row + wall.dir.1 == pos2.row)
        }) {
            self.is_open((pos1, wall))
        } else {
            false
        }
    }

    /// Sets whether a wall is open.
    ///
    /// # Arguments
    /// *  `wall_pos` - The wall position.
    /// *  `value` - Whether to open the wall.
    pub fn set_open(&mut self, wall_pos: WallPos, value: bool) {
        // First modify the requested wall...
        if let Some(room) = self.rooms.get_mut(wall_pos.0) {
            room.set_open(wall_pos.1, value);
        }

        // ...and then sync the value on the back
        let other = self.back(wall_pos);
        if let Some(other_room) = self.rooms.get_mut(other.0) {
            other_room.set_open(other.1, value);
        }
    }

    /// Opens a wall.
    ///
    /// # Arguments
    /// *  `wall_pos` - The wall position.
    pub fn open(&mut self, wall_pos: WallPos) {
        self.set_open(wall_pos, true);
    }

    /// Closes a wall.
    ///
    /// # Arguments
    /// *  `wall_pos` - The wall position.
    pub fn close(&mut self, wall_pos: WallPos) {
        self.set_open(wall_pos, false);
    }

    /// Returns an iterator over all rooms positions.
    ///
    /// The positions are returned row by row, starting from `(0, 0)` and ending
    /// with `(self.width() - 1, self.height - 1())`.
    pub fn positions(&self) -> impl Iterator<Item = matrix::Pos> {
        self.rooms.positions()
    }

    /// Returns the physical positions of the two corners of a wall.
    ///
    /// # Arguments
    /// *  `pos` - The matrix position.
    /// *  `wall` - The wall.
    pub fn corners(&self, wall_pos: WallPos) -> (physical::Pos, physical::Pos) {
        let center = self.center(wall_pos.0);
        (
            physical::Pos {
                x: center.x + wall_pos.1.span.0.dx,
                y: center.y + wall_pos.1.span.0.dy,
            },
            physical::Pos {
                x: center.x + wall_pos.1.span.1.dx,
                y: center.y + wall_pos.1.span.1.dy,
            },
        )
    }

    /// Returns all walls that meet in the corner where a wall has its start
    /// span.
    ///
    /// The walls are listed in counter-clockwise order. Only one side of each
    /// wall will be returned. Each consecutive wall will be in a room different
    /// from the previous.
    ///
    /// # Arguments
    /// *  `wall_pos` - The wall position.
    pub fn corner_walls(
        &self,
        wall_pos: WallPos,
    ) -> impl Iterator<Item = WallPos> {
        let (matrix::Pos { col, row }, wall) = wall_pos;
        let all = self.all_walls();
        std::iter::once(wall_pos).chain(
            all[wall.index].corner_wall_offsets.iter().map(
                move |&wall::Offset { dx, dy, wall }| {
                    (
                        matrix::Pos {
                            col: col + dx,
                            row: row + dy,
                        },
                        all[wall],
                    )
                },
            ),
        )
    }

    /// Iterates over all wall positions of a room.
    ///
    /// # Arguments
    /// *  `pos` - The room position.
    pub fn wall_positions<'a>(
        &'a self,
        pos: matrix::Pos,
    ) -> impl Iterator<Item = WallPos> + 'a {
        self.walls(pos).iter().map(move |&wall| (pos, wall))
    }

    /// Iterates over all open walls of a room.
    ///
    /// # Arguments
    /// *  `pos` - The room position.
    pub fn doors<'a>(
        &'a self,
        pos: matrix::Pos,
    ) -> impl Iterator<Item = &'static wall::Wall> + 'a {
        self.walls(pos)
            .iter()
            .filter(move |&wall| self.is_open((pos, wall)))
            .copied()
    }

    /// Iterates over all adjacent rooms.
    ///
    /// This method will list rooms outside of the maze for rooms on the edge.
    ///
    /// # Arguments
    /// *  `pos` - The room position.
    pub fn adjacent<'a>(
        &'a self,
        pos: matrix::Pos,
    ) -> impl Iterator<Item = matrix::Pos> + 'a {
        self.walls(pos).iter().map(move |&wall| matrix::Pos {
            col: pos.col + wall.dir.0,
            row: pos.row + wall.dir.1,
        })
    }

    /// Iterates over all reachble neighbours of a room.
    ///
    /// This method may list rooms outside of the maze if an opening outside
    /// exists.
    ///
    /// # Arguments
    /// *  `pos` - The room position.
    pub fn neighbors<'a>(
        &'a self,
        pos: matrix::Pos,
    ) -> impl Iterator<Item = matrix::Pos> + 'a {
        self.doors(pos).map(move |wall| self.back((pos, wall)).0)
    }
}

impl std::ops::Index<matrix::Pos> for Maze {
    type Output = room::Room;

    fn index(&self, pos: matrix::Pos) -> &Self::Output {
        &self.rooms[pos]
    }
}

/// A matrix of scores for rooms.
pub type HeatMap = matrix::Matrix<u32>;

/// Generates a heat map where the value for each cell is the number of times it
/// has been traversed when walking between the positions.
///
/// Any position pairs with no path between them will be ignored.
///
/// # Arguments
/// *  `positions` - The positions as the tuple `(from, to)`. These are used as
///   positions between which to walk.
pub fn heatmap<I>(maze: &crate::Maze, positions: I) -> HeatMap
where
    I: Iterator<Item = (matrix::Pos, matrix::Pos)>,
{
    let mut result = matrix::Matrix::new(maze.width(), maze.height());

    for (from, to) in positions {
        if let Some(path) = maze.walk(from, to) {
            for pos in path.into_iter() {
                result[pos] += 1;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use maze_test::maze_test;

    use super::test_utils::*;
    use super::*;

    #[maze_test]
    fn is_inside_correct(maze: Maze) {
        assert!(maze.is_inside(matrix_pos(0, 0)));
        assert!(maze.is_inside(matrix_pos(
            maze.width() as isize - 1,
            maze.height() as isize - 1,
        )));
        assert!(!maze.is_inside(matrix_pos(-1, -1)));
        assert!(!maze.is_inside(matrix_pos(
            maze.width() as isize,
            maze.height() as isize
        )));
    }

    #[maze_test]
    fn can_open(mut maze: Maze) {
        let log = Navigator::new(&mut maze).down(true).stop();
        let pos = log[0];
        let next = log[1];
        assert!(
            maze.walls(pos)
                .iter()
                .filter(|wall| maze.is_open((pos, wall)))
                .count()
                == 1
        );
        assert!(
            maze.walls(next)
                .iter()
                .filter(|wall| maze.is_open((next, wall)))
                .count()
                == 1
        );
    }

    #[maze_test]
    fn can_close(mut maze: Maze) {
        let log = Navigator::new(&mut maze).down(true).up(false).stop();
        let pos = log.first().unwrap();
        let next = log.last().unwrap();
        assert!(
            maze.walls(*pos)
                .iter()
                .filter(|wall| maze.is_open((*pos, wall)))
                .count()
                == 0
        );
        assert!(
            maze.walls(*next)
                .iter()
                .filter(|wall| maze.is_open((*next, wall)))
                .count()
                == 0
        );
    }

    #[maze_test]
    fn walls_correct(maze: Maze) {
        let walls = maze.walls(matrix_pos(0, 1));
        assert_eq!(
            walls
                .iter()
                .cloned()
                .collect::<HashSet<&wall::Wall>>()
                .len(),
            walls.len()
        );
    }

    #[maze_test]
    fn walls_span(maze: Maze) {
        for pos in maze.positions() {
            for wall in maze.walls(pos) {
                let d = (2.0 / 5.0) * (wall.span.1.a - wall.span.0.a);
                assert!(wall.in_span(wall.span.0.a + d));
                assert!(!wall.in_span(wall.span.0.a - d));
                assert!(wall.in_span(wall.span.1.a - d));
                assert!(!wall.in_span(wall.span.1.a + d));

                assert!(
                    nearly_equal(wall.span.0.a.cos(), wall.span.0.dx),
                    format!(
                        "{:?} wall {} span 0 dx invalid ({} != {})",
                        maze.shape(),
                        wall.name,
                        wall.span.0.a.cos(),
                        wall.span.0.dx,
                    ),
                );
                assert!(
                    nearly_equal(wall.span.0.a.sin(), wall.span.0.dy),
                    format!(
                        "{:?} wall {} span 0 dy invalid ({} != {})",
                        maze.shape(),
                        wall.name,
                        wall.span.0.a.sin(),
                        wall.span.0.dy,
                    ),
                );
                assert!(
                    nearly_equal(wall.span.1.a.cos(), wall.span.1.dx),
                    format!(
                        "{:?} wall {} span 1 dx invalid ({} != {})",
                        maze.shape(),
                        wall.name,
                        wall.span.1.a.cos(),
                        wall.span.1.dx,
                    ),
                );
                assert!(
                    nearly_equal(wall.span.1.a.sin(), wall.span.1.dy),
                    format!(
                        "{:?} wall {} span 1 dy invalid ({} != {})",
                        maze.shape(),
                        wall.name,
                        wall.span.1.a.sin(),
                        wall.span.1.dy,
                    ),
                );
            }
        }
    }

    #[maze_test]
    fn connecting_wall_correct(maze: Maze) {
        for pos in maze.positions() {
            for &wall in maze.walls(pos) {
                assert!(maze
                    .connecting_wall(
                        pos,
                        matrix::Pos {
                            col: pos.col - 3,
                            row: pos.row - 3
                        }
                    )
                    .is_none());
                let wall_pos = (pos, wall);
                let other = matrix::Pos {
                    col: pos.col + wall.dir.0,
                    row: pos.row + wall.dir.1,
                };
                assert_eq!(Some(wall_pos), maze.connecting_wall(pos, other));
            }
        }
    }

    #[maze_test]
    fn connected_correct(mut maze: Maze) {
        for pos in maze.positions() {
            assert!(maze.connected(pos, pos))
        }

        let pos1 = matrix_pos(1, 1);
        for wall in maze.walls(pos1) {
            let pos2 = matrix_pos(pos1.col + wall.dir.0, pos1.row + wall.dir.1);
            assert!(!maze.connected(pos1, pos2));
            maze.open((pos1, wall));
            assert!(maze.connected(pos1, pos2));
        }
    }

    #[maze_test]
    fn corner_walls(maze: Maze) {
        for pos in maze.positions() {
            for wall in maze.walls(pos) {
                let wall_pos = (pos, *wall);
                let (center, _) = maze.corners(wall_pos);
                for corner_wall in maze.corner_walls(wall_pos) {
                    let (start, end) = maze.corners(corner_wall);
                    assert!(is_close(start, center) || is_close(end, center));
                }
            }
        }
    }

    #[maze_test]
    fn doors(mut maze: Maze) {
        let pos = matrix::Pos { col: 0, row: 0 };
        assert_eq!(
            maze.doors(pos).collect::<Vec<&'static wall::Wall>>(),
            Vec::<&'static wall::Wall>::new(),
        );
        let walls = maze
            .walls(pos)
            .iter()
            .filter(|wall| maze.is_inside(maze.back((pos, wall)).0))
            .map(|&wall| wall)
            .collect::<Vec<_>>();
        walls.iter().for_each(|wall| maze.open((pos, wall)));
        assert_eq!(maze.doors(pos).collect::<Vec<_>>(), walls);
    }

    #[maze_test]
    fn adjacent(maze: Maze) {
        for pos1 in maze.positions() {
            for pos2 in maze.positions() {
                assert!(
                    maze.connecting_wall(pos1, pos2).is_some()
                        == maze.adjacent(pos1).find(|&p| pos2 == p).is_some()
                );
            }
        }
    }

    #[maze_test]
    fn neighbors(mut maze: Maze) {
        let pos = matrix::Pos { col: 0, row: 0 };
        assert_eq!(maze.neighbors(pos).collect::<Vec<_>>(), vec![]);
        maze.walls(pos)
            .iter()
            .for_each(|wall| maze.open((pos, wall)));
        assert_eq!(
            maze.neighbors(pos).collect::<Vec<_>>(),
            maze.walls(pos)
                .iter()
                .map(|wall| matrix::Pos {
                    col: pos.col + wall.dir.0,
                    row: pos.row + wall.dir.1
                })
                .collect::<Vec<_>>(),
        );
    }
}

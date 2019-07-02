#![cfg_attr(feature = "cargo-clippy", deny(clippy::all))]

use serde::{Deserialize, Serialize};

#[cfg(test)]
#[macro_use]
mod test_utils;

#[macro_use]
pub mod wall;

#[macro_use]
pub mod shape;

pub mod initialize;
pub mod matrix;
pub mod physical;
pub mod render;
pub mod room;
pub mod walk;

pub mod prelude;
pub use self::prelude::*;

mod util;

/// A wall of a room.
pub type WallPos = (matrix::Pos, &'static wall::Wall);

/// A maze contains rooms and has methods for managing paths and doors.
#[derive(Deserialize, Serialize)]
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
        Maze { shape, rooms }
    }

    /// Returns the width of the maze.
    ///
    /// This is short hand for `self.rooms.width()`.
    pub fn width(&self) -> usize {
        self.rooms.width
    }

    /// Returns the height of the maze.
    ///
    /// This is short hand for `self.rooms.height()`.
    pub fn height(&self) -> usize {
        self.rooms.height
    }

    /// Returns whether a specified wall is open.
    ///
    /// # Arguments
    /// * `wall_pos` - The wall position.
    pub fn is_open(&self, wall_pos: WallPos) -> bool {
        match self.rooms.get(wall_pos.0) {
            Some(room) => room.is_open(wall_pos.1),
            None => false,
        }
    }

    /// Returns whether two rooms are connected.
    ///
    /// Two rooms are connected if there is an open wall between them, or if
    /// they are the same room.
    ///
    /// # Arguments
    /// * `pos1` - The first room.
    /// * `pos2` - The second room.
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
    /// * `wall_pos` - The wall position.
    /// * `value` - Whether to open the wall.
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
    /// * `wall_pos` - The wall position.
    pub fn open(&mut self, wall_pos: WallPos) {
        self.set_open(wall_pos, true);
    }

    /// Closes a wall.
    ///
    /// # Arguments
    /// * `wall_pos` - The wall position.
    pub fn close(&mut self, wall_pos: WallPos) {
        self.set_open(wall_pos, false);
    }

    /// Retrieves a reference to the underlying rooms.
    pub fn rooms(&self) -> &room::Rooms {
        &self.rooms
    }

    /// Returns the physical positions of the two corners of a wall.
    ///
    /// # Arguments
    /// * `pos` - The matrix position.
    /// * `wall` - The wall.
    pub fn corners(&self, wall_pos: WallPos) -> (physical::Pos, physical::Pos) {
        let center = self.center(wall_pos.0);
        (
            physical::Pos {
                x: center.x + wall_pos.1.span.0.cos(),
                y: center.y + wall_pos.1.span.0.sin(),
            },
            physical::Pos {
                x: center.x + wall_pos.1.span.1.cos(),
                y: center.y + wall_pos.1.span.1.sin(),
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
    /// * `wall_pos` - The wall position.
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
            .map(|&wall| wall)
    }

    /// Iterates over all reachble neighbours of a room.
    ///
    /// This method may list rooms outside of the maze of an opening outside
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

/// A matrix of scores for rooms.
pub type HeatMap = matrix::Matrix<u32>;

/// Generates a heat map where the value for each cell is the number of times it
/// has been traversed when walking between the positions.
///
/// Any position pairs with no path between them will be ignored.
///
/// # Arguments
/// * `positions` - The positions as the tuple `(from, to)`. These are used as
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
    fn is_inside_correct(maze: &mut Maze) {
        assert!(maze.rooms.is_inside(matrix_pos(0, 0)));
        assert!(maze.rooms.is_inside(matrix_pos(
            maze.width() as isize - 1,
            maze.height() as isize - 1,
        )));
        assert!(!maze.rooms.is_inside(matrix_pos(-1, -1)));
        assert!(!maze.rooms.is_inside(matrix_pos(
            maze.width() as isize,
            maze.height() as isize
        )));
    }

    #[maze_test]
    fn can_open(maze: &mut Maze) {
        let log = Navigator::new(maze).down(true).stop();
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
    fn can_close(maze: &mut Maze) {
        let log = Navigator::new(maze).down(true).up(false).stop();
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
    fn walls_correct(maze: &mut Maze) {
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
    fn walls_span(maze: &mut Maze) {
        for pos in maze.rooms.positions() {
            for wall in maze.walls(pos) {
                let d = (2.0 / 5.0) * (wall.span.1 - wall.span.0);
                assert!(wall.in_span(wall.span.0 + d));
                assert!(!wall.in_span(wall.span.0 - d));
                assert!(wall.in_span(wall.span.1 - d));
                assert!(!wall.in_span(wall.span.1 + d));
            }
        }
    }

    #[maze_test]
    fn connected_correct(maze: &mut Maze) {
        for pos in maze.rooms.positions() {
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
    fn corner_walls(maze: &mut Maze) {
        for pos in maze.rooms.positions() {
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
    fn doors(maze: &mut Maze) {
        let pos = matrix::Pos { col: 0, row: 0 };
        assert_eq!(
            maze.doors(pos).collect::<Vec<&'static wall::Wall>>(),
            Vec::<&'static wall::Wall>::new(),
        );
        let walls = maze
            .walls(pos)
            .iter()
            .filter(|wall| maze.rooms().is_inside(maze.back((pos, wall)).0))
            .map(|&wall| wall)
            .collect::<Vec<_>>();
        walls.iter().for_each(|wall| maze.open((pos, wall)));
        assert_eq!(maze.doors(pos).collect::<Vec<_>>(), walls);
    }

    #[maze_test]
    fn neighbors(maze: &mut Maze) {
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

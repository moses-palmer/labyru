#![cfg_attr(feature = "cargo-clippy", deny(clippy::all))]

#[cfg(feature = "osrand")]
extern crate rand;

#[cfg(feature = "render-svg")]
extern crate svg;

#[cfg(test)]
#[macro_use]
mod test_utils;

#[macro_use]
pub mod wall;

#[macro_use]
pub mod shape;

pub mod traits;
pub use self::traits::*;

pub mod initialize;
pub mod matrix;
pub mod room;

pub mod prelude;
pub use self::prelude::*;

mod util;

/// A wall of a room.
pub type WallPos = (matrix::Pos, &'static wall::Wall);

/// A maze contains rooms and has methods for managing paths and doors.
pub trait Maze: shape::Shape + Physical + Renderable + Walkable + Sync {
    /// Returns the width of the maze.
    ///
    /// This is short hand for `self.rooms().width()`.
    fn width(&self) -> usize {
        self.rooms().width
    }

    /// Returns the height of the maze.
    ///
    /// This is short hand for `self.rooms().height()`.
    fn height(&self) -> usize {
        self.rooms().height
    }

    /// Returns whether a specified wall is open.
    ///
    /// # Arguments
    /// * `wall_pos` - The wall position.
    fn is_open(&self, wall_pos: WallPos) -> bool {
        match self.rooms().get(wall_pos.0) {
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
    fn connected(&self, pos1: matrix::Pos, pos2: matrix::Pos) -> bool {
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
    fn set_open(&mut self, wall_pos: WallPos, value: bool) {
        // First modify the requested wall...
        if let Some(room) = self.rooms_mut().get_mut(wall_pos.0) {
            room.set_open(wall_pos.1, value);
        }

        // ...and then sync the value on the back
        let other = self.back(wall_pos);
        if let Some(other_room) = self.rooms_mut().get_mut(other.0) {
            other_room.set_open(other.1, value);
        }
    }

    /// Opens a wall.
    ///
    /// # Arguments
    /// * `wall_pos` - The wall position.
    fn open(&mut self, wall_pos: WallPos) {
        self.set_open(wall_pos, true);
    }

    /// Closes a wall.
    ///
    /// # Arguments
    /// * `wall_pos` - The wall position.
    fn close(&mut self, wall_pos: WallPos) {
        self.set_open(wall_pos, false);
    }

    /// Retrieves a reference to the underlying rooms.
    fn rooms(&self) -> &room::Rooms;

    /// Retrieves a mutable reference to the underlying rooms.
    fn rooms_mut(&mut self) -> &mut room::Rooms;
}

/// The different types of mazes implemented, identified by number of walls.
pub enum MazeType {
    /// A maze with triangular rooms.
    Tri = 3,

    /// A maze with quadratic rooms.
    Quad = 4,

    /// A maze with hexagonal rooms.
    Hex = 6,
}

impl MazeType {
    /// Converts a number to a maze type.
    ///
    /// The number must be one of the known number of walls per room.
    ///
    /// # Arguments
    /// * `num ` - The number to convert.
    pub fn from_num(num: u32) -> Option<Self> {
        match num {
            x if x == MazeType::Tri as u32 => Some(MazeType::Tri),
            x if x == MazeType::Quad as u32 => Some(MazeType::Quad),
            x if x == MazeType::Hex as u32 => Some(MazeType::Hex),
            _ => None,
        }
    }

    /// Creates a maze of this type.
    ///
    /// # Arguments
    /// * `width` - The width, in rooms, of the maze.
    /// * `height` - The height, in rooms, of the maze.
    pub fn create(self, width: usize, height: usize) -> Box<Maze> {
        match self {
            MazeType::Tri => Box::new(shape::tri::Maze::new(width, height)),
            MazeType::Quad => Box::new(shape::quad::Maze::new(width, height)),
            MazeType::Hex => Box::new(shape::hex::Maze::new(width, height)),
        }
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
            for pos in path {
                result[pos] += 1;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::test_utils::*;
    use super::*;

    maze_test!(
        is_inside_correct,
        fn test(maze: &mut Maze) {
            assert!(maze.rooms().is_inside(matrix_pos(0, 0)));
            assert!(maze.rooms().is_inside(matrix_pos(
                maze.width() as isize - 1,
                maze.height() as isize - 1,
            )));
            assert!(!maze.rooms().is_inside(matrix_pos(-1, -1)));
            assert!(!maze.rooms().is_inside(matrix_pos(
                maze.width() as isize,
                maze.height() as isize
            )));
        }
    );

    maze_test!(
        can_open,
        fn test(maze: &mut Maze) {
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
    );

    maze_test!(
        can_close,
        fn test(maze: &mut Maze) {
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
    );

    maze_test!(
        walls_correct,
        fn test(maze: &mut Maze) {
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
    );

    maze_test!(
        walls_span,
        fn test(maze: &mut Maze) {
            for pos in maze.rooms().positions() {
                for wall in maze.walls(pos) {
                    let d = (2.0 / 5.0) * (wall.span.1 - wall.span.0);
                    assert!(wall.in_span(wall.span.0 + d));
                    assert!(!wall.in_span(wall.span.0 - d));
                    assert!(wall.in_span(wall.span.1 - d));
                    assert!(!wall.in_span(wall.span.1 + d));
                }
            }
        }
    );

    maze_test!(
        connected_correct,
        fn test(maze: &mut Maze) {
            for pos in maze.rooms().positions() {
                assert!(maze.connected(pos, pos))
            }

            let pos1 = matrix_pos(1, 1);
            for wall in maze.walls(pos1) {
                let pos2 =
                    matrix_pos(pos1.col + wall.dir.0, pos1.row + wall.dir.1);
                assert!(!maze.connected(pos1, pos2));
                maze.open((pos1, wall));
                assert!(maze.connected(pos1, pos2));
            }
        }
    );
}
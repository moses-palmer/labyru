#![cfg_attr(feature = "cargo-clippy", deny(clippy::all))]

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod test_utils;

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

/// A wall of a room.
pub type WallPos = (matrix::Pos, &'static wall::Wall);

/// A matrix of rooms.
type Rooms<T> = matrix::Matrix<room::Room<T>>;

/// A maze contains rooms and has methods for managing paths and doors.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Maze<T>
where
    T: Clone,
{
    /// The shape of the rooms.
    shape: Shape,

    /// The actual rooms.
    rooms: Rooms<T>,
}

impl<T> Maze<T>
where
    T: Clone + Default,
{
    /// Creates an uninitialised maze.
    ///
    /// # Arguments
    /// *  `shape` - The shape of the rooms.
    /// *  `width` - The width, in rooms, of the maze.
    /// *  `height` - The height, in rooms, of the maze.
    pub fn new(shape: Shape, width: usize, height: usize) -> Self {
        let rooms = Rooms::new(width, height);
        Self { shape, rooms }
    }
}

impl<T> Maze<T>
where
    T: Clone,
{
    /// Creates an uninitialised maze.
    ///
    /// This method allows creating a maze initialised with data.
    ///
    /// # Arguments
    /// *  `shape` - The shape of the rooms.
    /// *  `width` - The width, in rooms, of the maze.
    /// *  `height` - The height, in rooms, of the maze.
    /// *  `data` - A function providing room data.
    pub fn new_with_data<F>(
        shape: Shape,
        width: usize,
        height: usize,
        mut data: F,
    ) -> Self
    where
        F: FnMut(matrix::Pos) -> T,
    {
        let rooms = Rooms::new_with_data(width, height, |pos| data(pos).into());
        Self { shape, rooms }
    }

    /// Maps each room, yielding a maze with the same layout but with
    /// transformed data.
    ///
    /// # Arguments
    /// *  `data` - A function providing data for the new maze.
    pub fn map<F, U>(&self, mut data: F) -> Maze<U>
    where
        F: FnMut(matrix::Pos, T) -> U,
        U: Clone,
    {
        Maze {
            shape: self.shape,
            rooms: self.rooms.map_with_pos(|pos, value| {
                value.with_data(data(pos, value.data.clone()))
            }),
        }
    }

    /// The width of the maze.
    pub fn width(&self) -> usize {
        self.rooms.width
    }

    /// The height of the maze.
    pub fn height(&self) -> usize {
        self.rooms.height
    }

    /// The shape of the maze.
    pub fn shape(&self) -> Shape {
        self.shape
    }

    /// The data for a specific room.
    ///
    /// If the index is out of bounds, nothing is returned.
    ///
    /// # Arguments
    /// *  `pos``- The room position.
    pub fn data(&self, pos: matrix::Pos) -> Option<&T> {
        self.rooms.get(pos).map(|room| &room.data)
    }

    /// The mutable data for a specific room.
    ///
    /// If the position is out of bounds, nothing is returned.
    ///
    /// # Arguments
    /// *  `pos``- The room position.
    pub fn data_mut(&mut self, pos: matrix::Pos) -> Option<&mut T> {
        self.rooms.get_mut(pos).map(|room| &mut room.data)
    }

    /// Whether a position is inside of the maze.
    ///
    /// # Arguments
    /// *  `pos` - The romm position.
    pub fn is_inside(&self, pos: matrix::Pos) -> bool {
        self.rooms.is_inside(pos)
    }

    /// Whether a wall is open.
    ///
    /// If the position is out of bounds, `false` is returned.
    ///
    /// # Arguments
    /// *  `wall_pos` - The wall position.
    pub fn is_open(&self, wall_pos: WallPos) -> bool {
        self.rooms
            .get(wall_pos.0)
            .map(|room| room.is_open(wall_pos.1))
            .unwrap_or(false)
    }

    /// Finds the wall connecting two rooms.
    ///
    /// The returned wall position, if it exists, will be in the room at `pos1`.
    ///
    /// # Arguments
    /// *  `pos1` - The first room position.
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

    /// Whether two rooms are connected.
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

    /// Iterates over all room positions.
    ///
    /// The positions are visited row by row, starting from `(0, 0)` and ending
    /// with `(self.width() - 1, self.height - 1())`.
    pub fn positions(&self) -> impl Iterator<Item = matrix::Pos> {
        self.rooms.positions()
    }

    /// The physical positions of the two corners of a wall.
    ///
    /// # Arguments
    /// *  `wall_pos` - The wall position.
    pub fn corners(&self, wall_pos: WallPos) -> (physical::Pos, physical::Pos) {
        let center = self.center(wall_pos.0);
        (center + wall_pos.1.span.0, center + wall_pos.1.span.1)
    }

    /// All walls that meet in the corner where a wall has its start span.
    ///
    /// The walls are visited in counter-clockwise order. Only one side of each
    /// wall will be visited. Each consecutive wall will be in a room different
    /// from the previous one.
    ///
    /// This method will visit rooms outside of the maze for rooms on the edge.
    ///
    /// # Arguments
    /// *  `wall_pos` - The wall position.
    pub fn corner_walls(
        &self,
        wall_pos: WallPos,
    ) -> impl Iterator<Item = WallPos> + DoubleEndedIterator {
        let (matrix::Pos { col, row }, wall) = wall_pos;
        std::iter::once(wall_pos).chain(wall.corner_wall_offsets.iter().map(
            move |&wall::Offset { dx, dy, wall }| {
                (
                    matrix::Pos {
                        col: col + dx,
                        row: row + dy,
                    },
                    wall,
                )
            },
        ))
    }

    /// Iterates over all wall positions of a room.
    ///
    /// # Arguments
    /// *  `pos` - The room position.
    pub fn wall_positions<'a>(
        &'a self,
        pos: matrix::Pos,
    ) -> impl Iterator<Item = WallPos> + DoubleEndedIterator + 'a {
        self.walls(pos).iter().map(move |&wall| (pos, wall))
    }

    /// Iterates over all open walls of a room.
    ///
    /// # Arguments
    /// *  `pos` - The room position.
    pub fn doors<'a>(
        &'a self,
        pos: matrix::Pos,
    ) -> impl Iterator<Item = &'static wall::Wall> + DoubleEndedIterator + 'a
    {
        self.walls(pos)
            .iter()
            .filter(move |&wall| self.is_open((pos, wall)))
            .copied()
    }

    /// Iterates over all adjacent rooms.
    ///
    /// This method will visit rooms outside of the maze for rooms on the edge.
    ///
    /// # Arguments
    /// *  `pos` - The room position.
    pub fn adjacent<'a>(
        &'a self,
        pos: matrix::Pos,
    ) -> impl Iterator<Item = matrix::Pos> + DoubleEndedIterator + 'a {
        self.walls(pos).iter().map(move |&wall| matrix::Pos {
            col: pos.col + wall.dir.0,
            row: pos.row + wall.dir.1,
        })
    }

    /// Iterates over all reachable neighbours of a room.
    ///
    /// This method will visit rooms outside of the maze if an opening outside
    /// from the room exists.
    ///
    /// # Arguments
    /// *  `pos` - The room position.
    pub fn neighbors<'a>(
        &'a self,
        pos: matrix::Pos,
    ) -> impl Iterator<Item = matrix::Pos> + DoubleEndedIterator + 'a {
        self.doors(pos).map(move |wall| self.back((pos, wall)).0)
    }
}

impl<T> std::ops::Index<matrix::Pos> for Maze<T>
where
    T: Clone,
{
    type Output = room::Room<T>;

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
pub fn heatmap<I, T>(maze: &crate::Maze<T>, positions: I) -> HeatMap
where
    I: Iterator<Item = (matrix::Pos, matrix::Pos)>,
    T: Clone,
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
    use maze_test::maze_test;

    use super::test_utils::*;
    use super::*;

    #[test]
    fn data() {
        let mut maze = Shape::Quad.create::<bool>(5, 5);
        let pos = (0isize, 0isize).into();
        assert_eq!(Some(&false), maze.data(pos));
        *maze.data_mut(pos).unwrap() = true;
        assert_eq!(Some(&true), maze.data(pos));
    }

    #[maze_test]
    fn is_inside_correct(maze: TestMaze) {
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
    fn can_open(mut maze: TestMaze) {
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
    fn can_close(mut maze: TestMaze) {
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
    fn connecting_wall_correct(maze: TestMaze) {
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
    fn connected_correct(mut maze: TestMaze) {
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
    fn corner_walls(maze: TestMaze) {
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
    fn doors(mut maze: TestMaze) {
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
    fn adjacent(maze: TestMaze) {
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
    fn neighbors(mut maze: TestMaze) {
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

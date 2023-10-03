use std::f32::consts::SQRT_2;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::matrix;
use crate::physical;
use crate::wall;

use crate::{Maze, WallPos};

/// cos(30°)
const COS_30: f32 = 0.866_025_4f32;

/// sin(30°)
const SIN_30: f32 = 1.0 / 2.0;

/// cos(45°)
const COS_45: f32 = 0.5 * SQRT_2;

/// sin(45°)
const SIN_45: f32 = 0.5 * SQRT_2;

/// The different types of mazes implemented, identified by number of walls.
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "lowercase")
)]
pub enum Shape {
    /// A maze with triangular rooms.
    Tri = 3,

    /// A maze with quadratic rooms.
    Quad = 4,

    /// A maze with hexagonal rooms.
    Hex = 6,
}

impl Shape {
    /// Creates a maze of this type.
    ///
    /// # Arguments
    /// *  `width` - The width, in rooms, of the maze.
    /// *  `height` - The height, in rooms, of the maze.
    pub fn create<T>(self, width: usize, height: usize) -> Maze<T>
    where
        T: Clone + Default,
    {
        Maze::new(self, width, height)
    }

    /// Creates a fully initialised maze of this type.
    ///
    /// # Arguments
    /// *  `width` - The width, in rooms, of the maze.
    /// *  `height` - The height, in rooms, of the maze.
    /// *  `data` - A function providing data for rooms.
    pub fn create_with_data<F, T>(
        self,
        width: usize,
        height: usize,
        data: F,
    ) -> Maze<T>
    where
        F: FnMut(matrix::Pos) -> T,
        T: Clone,
    {
        Maze::new_with_data(self, width, height, data)
    }

    /// The number of walls per room for this shape.
    pub fn wall_count(self) -> usize {
        self as usize
    }

    /// Calculates the minimal dimensions for a maze to let the distance
    /// between the leftmost and rightmost corners be `width` and the distance
    /// between the top and bottom be `height`.
    ///
    /// # Arguments
    /// *  `width` - The required physical width.
    /// *  `height` - The required physical height.
    pub fn minimal_dimensions(self, width: f32, height: f32) -> (usize, usize) {
        dispatch!(self => minimal_dimensions(width, height))
    }

    /// Returns all walls for a shape.
    pub fn all_walls(self) -> &'static [&'static wall::Wall] {
        dispatch!(self => all_walls())
    }

    /// Returns the back of a wall.
    ///
    /// The back is the other side of the wall, located in a neighbouring room.
    ///
    /// # Arguments
    /// *  `wall_pos` - The wall position.
    pub fn back(self, wall_pos: WallPos) -> WallPos {
        dispatch!(self => back(wall_pos))
    }

    /// Returns the opposite of a wall.
    ///
    /// The opposite is the wall located on the opposite side of the room. For
    /// mazes with rooms with an odd number of walls, there is no opposite wall.
    ///
    /// # Arguments
    /// *  `wall_pos` - The wall position.
    pub fn opposite(self, wall_pos: WallPos) -> Option<&'static wall::Wall> {
        dispatch!(self => opposite(wall_pos))
    }

    /// Returns all walls of a specific room.
    ///
    /// # Arguments
    /// *  `pos` - The room position.
    pub fn walls(self, pos: matrix::Pos) -> &'static [&'static wall::Wall] {
        dispatch!(self => walls(pos))
    }

    /// Converts a physical position to a matrix cell.
    ///
    /// # Arguments
    /// *  `pos` - The physical position.
    pub fn physical_to_cell(self, pos: physical::Pos) -> matrix::Pos {
        dispatch!(self => physical_to_cell(pos))
    }

    /// Converts a physical position to a matrix cell.
    ///
    /// # Arguments
    /// *  `pos` - The physical position.
    pub fn physical_to_wall_pos(self, pos: physical::Pos) -> WallPos {
        dispatch!(self => physical_to_wall_pos(pos))
    }

    /// Returns the physical centre of a matrix cell.
    ///
    /// # Arguments
    /// *  `pos` - The matrix position.
    pub fn cell_to_physical(self, pos: matrix::Pos) -> physical::Pos {
        dispatch!(self => cell_to_physical(pos))
    }

    /// Calculates the _view box_ for a maze with this shape when rendered.
    ///
    /// The returned value is the minimal rectangle that will contain a maze
    /// with the specified matrix dimensions.
    ///
    /// # Arguments
    /// *  `cols` - The number of columns in the matrix.
    /// *  `rows` - The number of rows in the matrix.
    pub fn viewbox(self, cols: usize, rows: usize) -> physical::ViewBox {
        let mut window =
            (std::f32::MAX, std::f32::MAX, std::f32::MIN, std::f32::MIN);
        for y in 0..rows {
            let lpos = matrix::Pos {
                col: 0,
                row: y as isize,
            };
            let lcenter = self.cell_to_physical(lpos);
            let left = dispatch!(self => walls(lpos))
                .iter()
                .map(|wall| (lcenter, wall));

            let rpos = matrix::Pos {
                col: cols as isize - 1,
                row: y as isize,
            };
            let rcenter = self.cell_to_physical(rpos);
            let right = dispatch!(self => walls(rpos))
                .iter()
                .map(|wall| (rcenter, wall));

            window = left
                .chain(right)
                .map(|(center, wall)| {
                    (center.x + wall.span.0.dx, center.y + wall.span.0.dy)
                })
                .fold(window, |acc, v| {
                    (
                        acc.0.min(v.0),
                        acc.1.min(v.1),
                        acc.2.max(v.0),
                        acc.3.max(v.1),
                    )
                });
        }

        physical::ViewBox {
            corner: physical::Pos {
                x: window.0,
                y: window.1,
            },
            width: window.2 - window.0,
            height: window.3 - window.1,
        }
    }
}

impl TryFrom<u32> for Shape {
    type Error = u32;

    /// Attempts to convert a number to a shape.
    ///
    /// The number should indicate the number of walls for the shape.
    ///
    /// # Arguments
    /// *  `source` - The number of walls.
    fn try_from(source: u32) -> Result<Self, Self::Error> {
        match source {
            x if x == Shape::Tri as u32 => Ok(Shape::Tri),
            x if x == Shape::Quad as u32 => Ok(Shape::Quad),
            x if x == Shape::Hex as u32 => Ok(Shape::Hex),
            _ => Err(source),
        }
    }
}

impl std::fmt::Display for Shape {
    /// The opposite of [std::str::FromStr].
    ///
    /// # Examples
    ///
    /// ```
    /// # use maze::shape::*;
    ///
    /// assert_eq!(
    ///     Shape::Tri.to_string().parse::<Shape>(),
    ///     Ok(Shape::Tri),
    /// );
    /// assert_eq!(
    ///     Shape::Quad.to_string().parse::<Shape>(),
    ///     Ok(Shape::Quad),
    /// );
    /// assert_eq!(
    ///     Shape::Quad.to_string().parse::<Shape>(),
    ///     Ok(Shape::Quad),
    /// );
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Shape::*;
        match self {
            Tri => write!(f, "tri"),
            Quad => write!(f, "quad"),
            Hex => write!(f, "hex"),
        }
    }
}

impl std::str::FromStr for Shape {
    type Err = String;

    /// Converts a string to a maze type.
    ///
    /// The string must be one of the supported names, lower-cased.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::str::FromStr;
    /// # use maze::Shape;
    ///
    /// assert_eq!(
    ///     "hex".parse(),
    ///     Ok(Shape::Hex),
    /// );
    /// assert_eq!(
    ///     "quad".parse(),
    ///     Ok(Shape::Quad),
    /// );
    /// assert_eq!(
    ///     "tri".parse(),
    ///     Ok(Shape::Tri),
    /// );
    /// ```
    ///
    /// # Arguments
    /// *  `source` - The source string.
    fn from_str(source: &str) -> Result<Self, Self::Err> {
        match source {
            "tri" => Ok(Shape::Tri),
            "quad" => Ok(Shape::Quad),
            "hex" => Ok(Shape::Hex),
            e => Err(e.to_owned()),
        }
    }
}

impl<T> Maze<T>
where
    T: Clone,
{
    /// All walls for a shape.
    ///
    /// This method does not necessarily return an array where the length is
    /// equal to the number of walls, since walls for all room layouts are
    /// present.
    pub fn all_walls(&self) -> &'static [&'static wall::Wall] {
        self.shape.all_walls()
    }

    /// The back of a wall.
    ///
    /// The back is the other side of the wall, located in a neighbouring room.
    ///
    /// # Arguments
    /// *  `wall_pos` - The wall position.
    pub fn back(&self, wall_pos: WallPos) -> WallPos {
        self.shape.back(wall_pos)
    }

    /// The opposite of a wall.
    ///
    /// The opposite is the wall located on the opposite side of the room. For
    /// mazes with rooms with an odd number of walls, there is no opposite wall.
    ///
    /// # Arguments
    /// *  `wall_pos` - The wall position.
    pub fn opposite(&self, wall_pos: WallPos) -> Option<&'static wall::Wall> {
        self.shape.opposite(wall_pos)
    }

    /// All walls of a specific room.
    ///
    /// # Arguments
    /// *  `pos` - The room position.
    pub fn walls(&self, pos: matrix::Pos) -> &'static [&'static wall::Wall] {
        self.shape.walls(pos)
    }

    /// The physical centre of a matrix position.
    ///
    /// # Arguments
    /// *  `pos` - The matrix position.
    pub fn center(&self, pos: matrix::Pos) -> physical::Pos {
        self.shape.cell_to_physical(pos)
    }

    /// The matrix position whose centre is closest to a physical position.
    ///
    /// The position returned may not correspond to an actual room; it may lie
    /// outside of the maze.
    ///
    /// # Arguments
    /// *  `pos` - The physical position.
    pub fn room_at(&self, pos: physical::Pos) -> matrix::Pos {
        self.shape.physical_to_cell(pos)
    }

    /// The matrix position whose centre is closest to a physical position
    /// along with the closest wall.
    ///
    /// The position returned may not correspond to an actual room; it may lie
    /// outside of the maze.
    ///
    /// # Arguments
    /// *  `pos` - The physical position.
    pub fn wall_pos_at(&self, pos: physical::Pos) -> WallPos {
        self.shape.physical_to_wall_pos(pos)
    }

    /// Yields all rooms that are touched by the rectangle described.
    ///
    /// This method does not perform an exhaustive check; rather, only the
    /// centre and all corners of rooms are considered, and all rooms for which
    /// any of these points are inside of the rectangle are yielded.
    ///
    /// Thus, a small rectangle inside a room not touching the centre nor any
    /// corner will not match.
    ///
    /// # Arguments
    /// *  `viewbox` - The rectangle.
    pub fn rooms_touched_by(
        &self,
        viewbox: physical::ViewBox,
    ) -> Vec<matrix::Pos> {
        let center = viewbox.center();
        let left = viewbox.corner.x;
        let top = viewbox.corner.y;
        let right = left + viewbox.width;
        let bottom = top + viewbox.height;
        let start = self.room_at(center);

        let mut result = Vec::new();
        let mut distance = 0;
        loop {
            let before = result.len();

            // Add all rooms inside of the rectangle
            result.extend(surround(start, distance).filter(|&pos| {
                let center = self.center(pos);
                (center.x >= left
                    && center.y >= top
                    && center.x <= right
                    && center.y <= bottom)
                    || self
                        .walls(pos)
                        .iter()
                        .map(|wall| physical::Pos {
                            x: center.x + wall.span.0.dx,
                            y: center.y + wall.span.0.dy,
                        })
                        .any(|pos| {
                            pos.x >= left
                                && pos.y >= top
                                && pos.x <= right
                                && pos.y <= bottom
                        })
            }));

            if result.len() == before {
                break;
            } else {
                distance += 1;
            }
        }

        result
    }
}

/// Iterates over all positions with a horisontal or vertical distance of
/// `distance` from `pos`.
///
/// Positions are visited clock-wise, starting with the row where the row values
/// are the smallest.
///
/// # Arguments
/// *  `pos` - The centre position.
/// *  `distance` - The distance from the centre.
pub fn surround(
    pos: matrix::Pos,
    distance: usize,
) -> impl Iterator<Item = matrix::Pos> {
    let distance = distance as isize;

    // Generate iterators over the edges; let bottom filter to avoid adding the
    // same row twice when distance == 0
    let top = (pos.col - distance..=pos.col + distance)
        .map(move |col| (col, pos.row - distance).into());
    let bottom = (pos.col - distance..=pos.col + distance)
        .filter(move |_| distance != 0)
        .map(move |col| (col, pos.row + distance).into())
        .rev();
    let left = (pos.row - distance + 1..pos.row + distance)
        .map(move |row| (pos.col - distance, row).into())
        .rev();
    let right = (pos.row - distance + 1..pos.row + distance)
        .map(move |row| (pos.col + distance, row).into());

    top.chain(right).chain(bottom).chain(left)
}

pub mod hex;
pub mod quad;
pub mod tri;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use maze_test::maze_test;

    use super::*;
    use crate::*;
    use test_utils::*;

    #[test]
    fn surround_single() {
        assert_eq!(
            [(0isize, 0isize).into()]
                .iter()
                .cloned()
                .collect::<HashSet<matrix::Pos>>(),
            surround((0isize, 0isize).into(), 0).collect(),
        );
    }

    #[test]
    fn surround_multiple() {
        assert_eq!(
            [
                (-1isize, -1isize).into(),
                (0isize, -1isize).into(),
                (1isize, -1isize).into(),
                (-1isize, 0isize).into(),
                (1isize, 0isize).into(),
                (-1isize, 1isize).into(),
                (0isize, 1isize).into(),
                (1isize, 1isize).into(),
            ]
            .iter()
            .cloned()
            .collect::<HashSet<matrix::Pos>>(),
            surround((0isize, 0isize).into(), 1).collect(),
        );
    }

    #[test]
    fn viewbox_centered_at() {
        assert_eq!(
            physical::ViewBox::centered_at(
                physical::Pos { x: 0.0, y: 0.0 },
                2.0,
                2.0
            ),
            physical::ViewBox {
                corner: physical::Pos { x: -1.0, y: -1.0 },
                width: 2.0,
                height: 2.0,
            },
        );
    }

    #[test]
    fn viewbox_expand() {
        assert_eq!(
            physical::ViewBox {
                corner: physical::Pos { x: 1.0, y: 1.0 },
                width: 1.0,
                height: 1.0,
            }
            .expand(1.0),
            physical::ViewBox {
                corner: physical::Pos { x: 0.0, y: 0.0 },
                width: 3.0,
                height: 3.0,
            },
        );
        assert_eq!(
            physical::ViewBox {
                corner: physical::Pos { x: 1.0, y: 1.0 },
                width: 1.0,
                height: 1.0,
            }
            .expand(1.0)
            .expand(-1.0),
            physical::ViewBox {
                corner: physical::Pos { x: 1.0, y: 1.0 },
                width: 1.0,
                height: 1.0,
            },
        );
    }

    #[test]
    fn viewbox_center() {
        let center = physical::Pos { x: 5.0, y: -5.0 };
        assert_eq!(
            physical::ViewBox::centered_at(center, 10.0, 10.0).center(),
            center
        );
    }

    #[test]
    fn shape_from_str() {
        assert_eq!("tri".parse(), Ok(Shape::Tri),);
        assert_eq!("quad".parse(), Ok(Shape::Quad),);
        assert_eq!("hex".parse(), Ok(Shape::Hex),);
        assert_eq!("invalid".parse::<Shape>(), Err("invalid".to_owned()));
    }

    #[maze_test]
    fn create_with_data(maze: TestMaze) {
        let width = 10;
        let height = 5;

        let maze = maze
            .shape()
            .create_with_data(width, height, |pos| pos.col * pos.row);
        assert_eq!(maze.width(), width);
        assert_eq!(maze.height(), height);
        for pos in maze.positions() {
            assert_eq!(maze.data(pos), Some(&(pos.col * pos.row)));
        }
    }

    #[maze_test]
    fn minimal_dimensions(maze: TestMaze) {
        for i in 1..20 {
            let width = i as f32 * 0.5;
            let height = width;
            let (w, h) = maze.shape.minimal_dimensions(width, height);

            let m = maze.shape.create::<()>(w, h);
            let physical::ViewBox {
                width: actual_width,
                height: actual_height,
                ..
            } = m.viewbox();
            assert!(actual_width >= width);
            assert!(actual_height >= height);

            if w > 1 && h > 1 {
                let m = maze.shape.create::<()>(w - 1, h - 1);
                let physical::ViewBox {
                    width: actual_width,
                    height: actual_height,
                    ..
                } = m.viewbox();
                assert!(actual_width <= width);
                assert!(actual_height <= height);
            }
        }
    }

    #[maze_test]
    fn room_at(maze: TestMaze) {
        let d = 0.95;
        for pos in maze.positions() {
            let center = maze.center(pos);
            for wall in maze.walls(pos) {
                let x = center.x + d * wall.span.0.dx;
                let y = center.y + d * wall.span.0.dy;
                assert_eq!(maze.room_at(physical::Pos { x, y }), pos);
                assert_eq!(
                    maze.shape().physical_to_cell(physical::Pos { x, y }),
                    pos,
                );
            }
        }
    }

    #[maze_test]
    fn wall_pos_at(maze: TestMaze) {
        let steps = 10;

        for pos in maze.positions() {
            let center = maze.center(pos);
            for i in 0..steps {
                let a = 2.0 * std::f32::consts::PI * (i as f32 / steps as f32);
                let expected = (
                    pos,
                    maze.walls(pos)
                        .iter()
                        .cloned()
                        .filter(|wall| wall.in_span(a))
                        .next()
                        .unwrap(),
                );
                for r in &[0.1, 0.3, 0.5] {
                    assert_eq!(
                        expected,
                        maze.wall_pos_at(physical::Pos {
                            x: center.x + r * a.cos(),
                            y: center.y + r * a.sin(),
                        }),
                        "Invalid wall for {}°",
                        360.0 * a / (2.0 * std::f32::consts::PI),
                    );
                }
            }
        }
    }

    #[maze_test]
    fn rooms_touched_by_for_center(maze: TestMaze) {
        let (left, top, right, bottom) = maze
            .positions()
            .filter(|pos| pos.row == 0)
            .map(|pos| maze.center(pos))
            .fold(
                (std::f32::MAX, std::f32::MAX, std::f32::MIN, std::f32::MIN),
                |(l, t, r, b), p| {
                    (l.min(p.x), t.min(p.y), r.max(p.x), b.max(p.y))
                },
            );
        let viewbox = physical::ViewBox {
            corner: physical::Pos { x: left, y: top },
            width: right - left,
            height: bottom - top,
        };

        assert_eq!(
            maze.positions()
                .filter(|pos| pos.row == 0)
                .collect::<HashSet<_>>(),
            maze.rooms_touched_by(viewbox)
                .into_iter()
                .filter(|&pos| maze.is_inside(pos))
                .collect::<HashSet<_>>(),
        );
    }

    #[maze_test]
    fn rooms_touched_by_for_corners(maze: TestMaze) {
        let (left, top, right, bottom) = maze
            .positions()
            .filter(|pos| pos.row == 0)
            .flat_map(|pos| {
                let center = maze.center(pos);
                maze.walls(pos).iter().map(move |wall| physical::Pos {
                    x: center.x + wall.span.0.dx,
                    y: center.y + wall.span.0.dy,
                })
            })
            .fold(
                (std::f32::MAX, std::f32::MAX, std::f32::MIN, std::f32::MIN),
                |(l, t, r, b), p| {
                    (l.min(p.x), t.min(p.y), r.max(p.x), b.max(p.y))
                },
            );
        let viewbox = physical::ViewBox {
            corner: physical::Pos { x: left, y: top },
            width: right - left,
            height: bottom - top,
        };

        assert_eq!(
            maze.positions()
                .filter(|pos| pos.row == 0 || pos.row == 1)
                .collect::<HashSet<_>>(),
            maze.rooms_touched_by(viewbox)
                .into_iter()
                .filter(|&pos| maze.is_inside(pos))
                .collect::<HashSet<_>>(),
        );
    }

    #[maze_test]
    fn previous_and_next_wall(maze: TestMaze) {
        for pos in maze.positions() {
            for wall in maze.walls(pos) {
                let a1 = wall::Wall::normalized_angle(wall.span.0.a);
                let a2 = wall::Wall::normalized_angle(wall.previous.span.1.a);
                assert!(
                    (a1 - a2).abs() < std::f32::EPSILON * 16.0,
                    "first wall {:?} for {:?} ({} != {})",
                    wall,
                    maze.shape(),
                    a1,
                    a2,
                );
                let a1 = wall::Wall::normalized_angle(wall.span.1.a);
                let a2 = wall::Wall::normalized_angle(wall.next.span.0.a);
                assert!(
                    (a1 - a2).abs() < std::f32::EPSILON * 16.0,
                    "second wall {:?} for {:?} ({} != {})",
                    wall,
                    maze.shape(),
                    a1,
                    a2,
                );
            }
        }
    }
}

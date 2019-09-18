use std;

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
const COS_45: f32 = 0.707_106_77f32;

/// sin(45°)
const SIN_45: f32 = 0.707_106_77f32;

/// Dispatches a function call for the current maze to a shape defined module.
macro_rules! dispatch {
    ($on:expr => $func:ident ( $($args:ident $(,)?)* ) ) => {
        match $on {
            crate::Shape::Hex => hex::$func($($args,)*),
            crate::Shape::Quad => quad::$func($($args,)*),
            crate::Shape::Tri => tri::$func($($args,)*),
        }
    }
}

/// Defines a wall module.
///
/// This is an internal library macro.
macro_rules! define_shape {
    ( $( $wall_name:ident = { $( $field:ident: $val:expr, )* } ),* ) => {
        #[allow(unused_imports, non_camel_case_types)]
        pub mod walls {
            use $crate::wall as wall;
            use super::*;

            pub enum WallIndex {
                $($wall_name,)*
            }

            $(pub static $wall_name: wall::Wall = wall::Wall {
                name: stringify!($wall_name),
                index: WallIndex::$wall_name as usize,
                $( $field: $val, )*
            } );*;

            pub static ALL: &[&'static wall::Wall] = &[
                            $(&$wall_name),*];
        }

        /// Returns all walls used in this type of maze.
        pub fn all_walls() -> &'static [&'static wall::Wall] {
            &walls::ALL
        }

        /// Returns the wall on the back of `wall_pos`.
        ///
        /// # Arguments
        /// *  `wall_pos` - The wall for which to find the back.
        pub fn back(wall_pos: WallPos) -> WallPos {
            let (pos, wall) = wall_pos;
            let other = matrix::Pos {
                col: pos.col + wall.dir.0,
                row: pos.row + wall.dir.1,
            };

            (other, walls::ALL[self::back_index(wall.index)])
        }
    }
}

/// The different types of mazes implemented, identified by number of walls.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
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
    pub fn create(self, width: usize, height: usize) -> Maze {
        Maze::new(self, width, height)
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
}

impl std::convert::TryFrom<u32> for Shape {
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

impl std::str::FromStr for Shape {
    type Err = String;

    /// Converts a string to a maze type.
    ///
    /// The string must be one of the supported names, lower-cased.
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

impl Maze {
    /// Returns all walls for a shape.
    pub fn all_walls(&self) -> &'static [&'static wall::Wall] {
        dispatch!(self.shape => all_walls())
    }

    /// Returns the back of a wall.
    ///
    /// The back is the other side of the wall, located in a neighbouring room.
    ///
    /// # Arguments
    /// *  `wall_pos` - The wall position.
    pub fn back(&self, wall_pos: WallPos) -> WallPos {
        dispatch!(self.shape => back(wall_pos))
    }

    /// Returns the opposite of a wall.
    ///
    /// The opposite is the wall located on the opposite side of the room. For
    /// mazes with rooms with an odd number of walls, there is no opposite wall.
    ///
    /// # Arguments
    /// *  `wall_pos` - The wall position.
    pub fn opposite(&self, wall_pos: WallPos) -> Option<&'static wall::Wall> {
        dispatch!(self.shape => opposite(wall_pos))
    }

    /// Returns all walls of a specific room.
    ///
    /// # Arguments
    /// *  `pos` - The room position.
    pub fn walls(&self, pos: matrix::Pos) -> &'static [&'static wall::Wall] {
        dispatch!(self.shape => walls(pos))
    }

    /// Returns the physical centre of a matrix position.
    ///
    /// # Arguments
    /// *  `pos` - The matrix position.
    pub fn center(&self, pos: matrix::Pos) -> physical::Pos {
        dispatch!(self.shape => center(pos))
    }

    /// Returns the matrix position whose centre is closest to a physical
    /// position.
    ///
    /// The position returned may not correspond to an actual room; it may lie
    /// outside of the maze.
    ///
    /// # Arguments
    /// *  `pos` - The physical position.
    pub fn room_at(&self, pos: physical::Pos) -> matrix::Pos {
        dispatch!(self.shape => room_at(pos))
    }

    /// Yields all rooms that are touched by the rectangle described.
    ///
    /// This method does not perform an exhaustive check; rather, only the
    /// centre and all corners of rooms are considered, and all rooms for which
    /// any of these points are inside of the rectangle are yielded.
    ///
    /// This, a small rectangle inside a room, but not touching the centre nor
    /// any corner, will not match.
    ///
    /// # Arguments
    /// *  `center` - The centre of the rectangle.
    /// *  `width` - The width of the rectangle. The absolute value will be
    ///    used.
    /// *  `height` - The height of the rectangle. The absolute value will be
    ///    used.
    pub fn rooms_touched_by(
        &self,
        center: physical::Pos,
        width: f32,
        height: f32,
    ) -> Vec<matrix::Pos> {
        let dx = (width * 0.5).abs();
        let dy = (height * 0.5).abs();
        let top = center.y - dy;
        let left = center.x - dx;
        let bottom = center.y + dy;
        let right = center.x + dx;
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

/// Yields all positions with a horisontal or vertical distance of `distance`
/// from `pos`.
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
    let top = (pos.col - distance..pos.col + distance + 1)
        .map(move |col| (col, pos.row - distance).into());
    let bottom = (pos.col - distance..pos.col + distance + 1)
        .filter(move |_| distance != 0)
        .map(move |col| (col, pos.row + distance).into());
    let left = (pos.row - distance + 1..pos.row + distance)
        .map(move |row| (pos.col - distance, row).into());
    let right = (pos.row - distance + 1..pos.row + distance)
        .map(move |row| (pos.col + distance, row).into());

    top.chain(bottom).chain(left).chain(right)
}

pub mod hex;
pub mod quad;
pub mod tri;

#[cfg(test)]
mod tests {
    use std::collections::hash_set;

    use maze_test::maze_test;

    use super::*;
    use crate::*;

    #[test]
    fn surround_single() {
        assert_eq!(
            [(0isize, 0isize).into()]
                .iter()
                .cloned()
                .collect::<hash_set::HashSet<matrix::Pos>>(),
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
            .collect::<hash_set::HashSet<matrix::Pos>>(),
            surround((0isize, 0isize).into(), 1).collect(),
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
    fn minimal_dimensions(maze: Maze) {
        for i in 1..20 {
            let width = i as f32 * 0.5;
            let height = width;
            let (w, h) = maze.shape.minimal_dimensions(width, height);

            let m = maze.shape.create(w, h);
            let (_, _, actual_width, actual_height) = m.viewbox();
            assert!(actual_width >= width);
            assert!(actual_height >= height);

            if w > 1 && h > 1 {
                let m = maze.shape.create(w - 1, h - 1);
                let (_, _, actual_width, actual_height) = m.viewbox();
                assert!(actual_width <= width);
                assert!(actual_height <= height);
            }
        }
    }

    #[maze_test]
    fn room_at(maze: Maze) {
        let d = 0.95;
        for pos in maze.positions() {
            let center = maze.center(pos);
            for wall in maze.walls(pos) {
                let x = center.x + d * wall.span.0.dx;
                let y = center.y + d * wall.span.0.dy;
                assert_eq!(maze.room_at(physical::Pos { x, y }), pos);
            }
        }
    }

    #[maze_test]
    fn rooms_touched_by_for_center(maze: Maze) {
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
        let center = physical::Pos {
            x: (left + right) / 2.0,
            y: (top + bottom) / 2.0,
        };
        let width = right - left;
        let height = bottom - top;

        assert_eq!(
            maze.positions()
                .filter(|pos| pos.row == 0)
                .collect::<hash_set::HashSet<_>>(),
            maze.rooms_touched_by(center, width, height)
                .into_iter()
                .filter(|&pos| maze.is_inside(pos))
                .collect::<hash_set::HashSet<_>>(),
        );
    }

    #[maze_test]
    fn rooms_touched_by_for_corners(maze: Maze) {
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
        let center = physical::Pos {
            x: (left + right) / 2.0,
            y: (top + bottom) / 2.0,
        };
        let width = right - left;
        let height = bottom - top;

        assert_eq!(
            maze.positions()
                .filter(|pos| pos.row == 0 || pos.row == 1)
                .collect::<hash_set::HashSet<_>>(),
            maze.rooms_touched_by(center, width, height)
                .into_iter()
                .filter(|&pos| maze.is_inside(pos))
                .collect::<hash_set::HashSet<_>>(),
        );
    }
}

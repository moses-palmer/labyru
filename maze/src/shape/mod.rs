use std;

use crate::matrix;
use crate::physical;
use crate::wall;

use crate::{Maze, WallPos};

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
#[derive(Clone, Copy, Debug, PartialEq)]
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
    /// * `width` - The width, in rooms, of the maze.
    /// * `height` - The height, in rooms, of the maze.
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
    /// * `wall_pos` - The wall position.
    pub fn back(&self, wall_pos: WallPos) -> WallPos {
        dispatch!(self.shape => back(wall_pos))
    }

    /// Returns the opposite of a wall.
    ///
    /// The opposite is the wall located on the opposite side of the room. For
    /// mazes with rooms with an odd number of walls, there is no opposite wall.
    ///
    /// # Arguments
    /// * `wall_pos` - The wall position.
    pub fn opposite(&self, wall_pos: WallPos) -> Option<&'static wall::Wall> {
        dispatch!(self.shape => opposite(wall_pos))
    }

    /// Returns all walls of a specific room.
    ///
    /// # Arguments
    /// * `pos` - The room position.
    pub fn walls(&self, pos: matrix::Pos) -> &'static [&'static wall::Wall] {
        dispatch!(self.shape => walls(pos))
    }

    /// Returns the physical centre of a matrix position.
    ///
    /// # Arguments
    /// * `pos` - The matrix position.
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
    /// * `pos` - The physical position.
    pub fn room_at(&self, pos: physical::Pos) -> matrix::Pos {
        dispatch!(self.shape => room_at(pos))
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
    pub fn corner_walls(&self, wall_pos: WallPos) -> Vec<WallPos> {
        let (matrix::Pos { col, row }, wall) = wall_pos;
        let all = self.all_walls();
        std::iter::once(wall_pos)
            .chain(all[wall.index].corner_wall_offsets.iter().map(
                |&((dx, dy), wall)| {
                    (
                        matrix::Pos {
                            col: col + dx,
                            row: row + dy,
                        },
                        all[wall],
                    )
                },
            ))
            .collect()
    }
}

pub mod hex;
pub mod quad;
pub mod tri;

/// Partitions a number into its integral part and a fraction.
///
/// The fraction indicates the distance through the integral to the next
/// greater number.
///
/// # Arguments
/// *  `x` - a number.
fn partition(x: f32) -> (isize, f32) {
    let index = x.floor() as isize;
    let rel = x.fract();
    (index, if x >= 0.0 { rel } else { rel + 1.0 })
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;
    use crate::*;

    #[test]
    fn shape_from_str() {
        assert_eq!("tri".parse(), Ok(Shape::Tri),);
        assert_eq!("quad".parse(), Ok(Shape::Quad),);
        assert_eq!("hex".parse(), Ok(Shape::Hex),);
        assert_eq!("invalid".parse::<Shape>(), Err("invalid".to_owned()));
    }

    #[test]
    fn partition() {
        let (index, rel) = super::partition(1.2);
        assert_eq!(index, 1);
        assert!((rel - 0.2).abs() < 0.0001);

        let (index, rel) = super::partition(-1.2);
        assert_eq!(index, -2);
        assert!((rel - 0.8).abs() < 0.0001);
    }

    maze_test!(
        minimal_dimensions,
        fn test(maze: &mut Maze) {
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
    );

    maze_test!(
        corner_walls,
        fn test(maze: &mut Maze) {
            for pos in maze.rooms.positions() {
                for wall in maze.walls(pos) {
                    let wall_pos = (pos, *wall);
                    let (center, _) = maze.corners(wall_pos);
                    for corner_wall in maze.corner_walls(wall_pos) {
                        let (start, end) = maze.corners(corner_wall);
                        assert!(
                            is_close(start, center) || is_close(end, center)
                        );
                    }
                }
            }
        }
    );

    maze_test!(
        room_at,
        fn test(maze: &mut Maze) {
            let d = 0.95;
            for pos in maze.rooms.positions() {
                let center = maze.center(pos);
                for wall in maze.walls(pos) {
                    let a = wall.span.0;
                    let x = center.x + d * a.cos();
                    let y = center.y + d * a.sin();
                    assert_eq!(maze.room_at(physical::Pos { x, y }), pos);
                }
            }
        }
    );
}

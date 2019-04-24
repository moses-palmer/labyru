use std;

use crate::matrix;
use crate::physical;
use crate::wall;

use crate::{Maze, WallPos};

/// Dispatches a function call for the current maze to a shape defined module.
macro_rules! dispatch {
    ($on:ident: $func:ident ( $($args:ident $(,)?)* ) ) => {
        match $on.shape {
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

impl Maze {
    /// Returns all walls for a shape.
    pub fn all_walls(&self) -> &'static [&'static wall::Wall] {
        dispatch!(self: all_walls())
    }

    /// Returns the back of a wall.
    ///
    /// The back is the other side of the wall, located in a neighbouring room.
    ///
    /// # Arguments
    /// * `wall_pos` - The wall position.
    pub fn back(&self, wall_pos: WallPos) -> WallPos {
        dispatch!(self: back(wall_pos))
    }

    /// Returns the opposite of a wall.
    ///
    /// The opposite is the wall located on the opposite side of the room. For
    /// mazes with rooms with an odd number of walls, there is no opposite wall.
    ///
    /// # Arguments
    /// * `wall_pos` - The wall position.
    pub fn opposite(&self, wall_pos: WallPos) -> Option<&'static wall::Wall> {
        dispatch!(self: opposite(wall_pos))
    }

    /// Returns all walls of a specific room.
    ///
    /// # Arguments
    /// * `pos` - The room position.
    pub fn walls(&self, pos: matrix::Pos) -> &'static [&'static wall::Wall] {
        dispatch!(self: walls(pos))
    }

    /// Returns the physical centre of a matrix position.
    ///
    /// # Arguments
    /// * `pos` - The matrix position.
    pub fn center(&self, pos: matrix::Pos) -> physical::Pos {
        dispatch!(self: center(pos))
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
        dispatch!(self: room_at(pos))
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

#[cfg(test)]
mod tests {
    use crate::test_utils::*;
    use crate::*;

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
            let (left, top, width, height) = maze.viewbox();
            let physical::Pos { x: min_x, y: min_y } =
                maze.center(matrix_pos(0, 0));
            let physical::Pos { x: max_x, y: max_y } = maze.center(matrix_pos(
                maze.width() as isize - 1,
                maze.height() as isize - 1,
            ));
            let xres = 100usize;
            let yres = 100usize;
            for x in 0..xres {
                for y in 0..yres {
                    let pos = physical::Pos {
                        x: x as f32 / (xres as f32 * width + left),
                        y: y as f32 / (yres as f32 * height + top),
                    };

                    // Should this position be inside the maze?
                    let assume_inside = true
                        && pos.x >= min_x
                        && pos.x <= max_x
                        && pos.y >= min_y
                        && pos.y <= max_y;

                    // Ignore rooms outside of the maze since we use
                    // maze.rooms.positions() below
                    let actual = maze.room_at(pos);
                    if !maze.rooms.is_inside(actual) && !assume_inside {
                        continue;
                    }

                    let mut positions = maze
                        .rooms
                        .positions()
                        .map(|matrix_pos| (maze.center(matrix_pos), matrix_pos))
                        .map(|(physical_pos, matrix_pos)| {
                            (distance(pos, physical_pos), matrix_pos)
                        })
                        .collect::<Vec<_>>();
                    positions.sort_by_key(|&(k, _)| k);

                    let (_, expected) = positions[0];
                    assert_eq!(expected, actual);
                }
            }
        }
    );

    /// Calculates an integral distance value between two points.
    ///
    /// # Arguments
    /// * `pos1` - The first point.
    /// * `pos2` - The second point.
    fn distance(pos1: physical::Pos, pos2: physical::Pos) -> u64 {
        (10000000000.0 * true_distance(pos1, pos2)) as u64
    }

    /// Calculates the actual distance value between two points.
    ///
    /// # Arguments
    /// * `pos1` - The first point.
    /// * `pos2` - The second point.
    fn true_distance(pos1: physical::Pos, pos2: physical::Pos) -> f32 {
        let dx = pos1.x - pos2.x;
        let dy = pos1.y - pos2.y;
        (dx * dx + dy * dy).sqrt()
    }
}

use std;

use matrix;
use wall;

use WallPos;


/// Defines the base methods for a shape.
///
/// This macro allows defining constants that are stored in the maze struct and
/// initialised upon creation. This is a work-around until true constant
/// functions are introduced.
macro_rules! define_base {
    ($($field:ident: $type:ident = $value:expr,)*) => {
        pub struct Maze {
            rooms: room::Rooms,
            $($field: $type,)*
        }

        impl Maze {
            pub fn new(width: usize, height: usize) -> Maze {
                Maze {
                    rooms: room::Rooms::new(width, height),
                    $($field: $value,)*
                }
            }
        }

        impl ::Maze for Maze {
            fn rooms(&self) -> &room::Rooms {
                &self.rooms
            }

            fn rooms_mut(&mut self) -> &mut room::Rooms {
                &mut self.rooms
            }
        }
    }
}


pub trait Shape {
    /// Returns all walls for a shape.
    fn all_walls(&self) -> &'static [&'static wall::Wall];

    /// Returns the back of a wall.
    ///
    /// The back is the other side of the wall, located in a neighbouring room.
    ///
    /// # Arguments
    /// * `wall_pos` - The wall position.
    fn back(&self, wall_pos: WallPos) -> WallPos {
        let (pos, wall) = wall_pos;
        let other = (pos.0 + wall.dir.0, pos.1 + wall.dir.1);
        (other, self.opposite((other, wall)).unwrap())
    }

    /// Returns the opposite of a wall.
    ///
    /// The opposite is the wall located on the opposite side of the room. For
    /// mazes with rooms with an odd number of walls, there is no opposite wall.
    ///
    /// # Arguments
    /// * `wall_pos` - The wall position.
    fn opposite(&self, wall_pos: WallPos) -> Option<&'static wall::Wall>;

    /// Returns all walls of a specific room.
    ///
    /// # Arguments
    /// * `pos` - The room position.
    #[allow(unused_variables)]
    fn walls(&self, pos: matrix::Pos) -> &'static [&'static wall::Wall] {
        self.all_walls()
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
    fn corner_walls(&self, wall_pos: WallPos) -> Vec<WallPos> {
        let ((x, y), wall) = wall_pos;
        let all = self.all_walls();
        std::iter::once(wall_pos)
            .chain(all[wall.index].corner_wall_offsets.iter().map(|&((dx, dy),
               wall)| {
                ((x + dx, y + dy), all[wall])
            }))
            .collect()
    }
}


pub mod quad;

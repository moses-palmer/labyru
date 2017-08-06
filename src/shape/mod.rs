use matrix;
use wall;

use WallPos;

pub mod quad;


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
}

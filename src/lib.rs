extern crate rand;

pub mod matrix;
pub mod room;

#[macro_use]
pub mod wall;

mod open_set;


/// A wall of a room.
pub type WallPos = (matrix::Pos, &'static wall::Wall);


/// A maze contains rooms and has methods for managing paths and doors.
pub trait Maze: walker::Walkable {
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
    /// * `pos` - The room position.
    /// * `wall` - The wall to check.
    fn is_open(&self, pos: matrix::Pos, wall: &'static wall::Wall) -> bool {
        match self.rooms().get(pos) {
            Some(room) => room.is_open(wall),
            None => false,
        }
    }

    /// Sets whether a wall is open.
    ///
    /// # Arguments
    /// * `pos` - The room position.
    /// * `wall` - The wall to modify.
    /// * `value` - Whether to open the wall.
    fn set_open(
        &mut self,
        pos: matrix::Pos,
        wall: &'static wall::Wall,
        value: bool,
    ) {
        // First modify the requested wall...
        if let Some(room) = self.rooms_mut().get_mut(pos) {
            room.set_open(wall, value);
        }

        // ..and then sync the value on the back
        let (other_pos, other_wall) = self.back(pos, wall);
        if let Some(other_room) = self.rooms_mut().get_mut(other_pos) {
            other_room.set_open(other_wall, value);
        }
    }

    /// Opens a wall.
    ///
    /// # Arguments
    /// * `pos` - The room position.
    /// * `wall` - The wall to open.
    fn open(&mut self, pos: matrix::Pos, wall: &'static wall::Wall) {
        self.set_open(pos, wall, true);
    }

    /// Closes a wall.
    ///
    /// # Arguments
    /// * `pos` - The room position.
    /// * `wall` - The wall to close.
    fn close(&mut self, pos: matrix::Pos, wall: &'static wall::Wall) {
        self.set_open(pos, wall, false);
    }

    /// Returns the back of a wall.
    ///
    /// The back is the other side of the wall, located in a neighbouring room.
    ///
    /// # Arguments
    /// * `pos` - The room position.
    /// * `wall` - The wall.
    fn back(&self, pos: matrix::Pos, wall: &'static wall::Wall) -> WallPos {
        let other = (pos.0 + wall.dx, pos.1 + wall.dy);
        (other, self.opposite(other, wall).unwrap())
    }

    /// Returns the opposite of a wall.
    ///
    /// The opposite is the wall located on the opposite side of the room. For
    /// mazes with rooms with an odd number of walls, there is no opposite wall.
    ///
    /// # Arguments
    /// * `pos` - The room position.
    /// * `wall` - The wall.
    fn opposite(
        &self,
        pos: matrix::Pos,
        wall: &'static wall::Wall,
    ) -> Option<&'static wall::Wall>;

    /// Returns all walls of a specific room.
    ///
    /// # Arguments
    /// * `pos` - The room position.
    fn walls(&self, pos: matrix::Pos) -> &'static [&'static wall::Wall];

    /// Retrieves a reference to the underlying rooms.
    fn rooms(&self) -> &room::Rooms;

    /// Retrieves a mutable reference to the underlying rooms.
    fn rooms_mut(&mut self) -> &mut room::Rooms;
}


#[cfg(test)]
#[macro_use]
mod tests;

pub mod initialize;
pub use initialize::*;
pub mod shape;
pub mod walker;
pub use walker::*;

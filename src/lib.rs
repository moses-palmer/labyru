extern crate ndarray;

pub mod room;

#[macro_use]
pub mod wall;

#[cfg(test)]
mod tests;


/// A room position.
///
/// The position is not an attribute of a [room](trait.Room.html), but a room
/// can be accessed from a [maze](../struct.Maze.html).
pub type Pos = (isize, isize);


trait Rooms<T>
    where T: Clone + Default
{
    /// The number of rooms across the maze, horizontally.
    fn width(&self) -> usize;

    /// The number of rooms across the maze, vertically.
    fn height(&self) -> usize;

    /// Determines whether a position is inside of the maze.
    ///
    /// # Arguments
    /// * `pos` - The room position.
    fn is_inside(&self, pos: Pos) -> bool {
        pos.0 >= 0 && pos.1 >= 0 && pos.0 < self.width() as isize &&
        pos.1 < self.height() as isize
    }


    /// Retrieves a reference the room at a specific position if it exists.
    ///
    /// # Arguments
    /// * `pos` - The room position.
    fn get(&self, pos: Pos) -> Option<&room::Room<T>>;

    /// Retrieves a mutable reference to the room at a specific position if it
    /// exists.
    ///
    /// # Arguments
    /// * `pos` - The room position.
    fn get_mut(&mut self, pos: Pos) -> Option<&mut room::Room<T>>;
}


/// A maze contains rooms and has methods for managing paths and doors.
trait Maze<T>: Rooms<T>
    where T: Clone + Default
{
    /// Returns whether a specified wall is open.
    ///
    /// # Arguments
    /// * `pos` - The room position.
    /// * `wall` - The wall to check.
    fn is_open(&self, pos: Pos, wall: &'static wall::Wall) -> bool {
        match self.get(pos) {
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
    fn set_open(&mut self, pos: Pos, wall: &'static wall::Wall, value: bool) {
        // First modify the requested wall...
        if let Some(room) = self.get_mut(pos) {
            room.set_open(wall, value);
        }

        // ..and then sync the value on the back
        let (other_pos, other_wall) = self.back(pos, wall);
        if let Some(other_room) = self.get_mut(other_pos) {
            other_room.set_open(other_wall, value);
        }
    }

    /// Opens a wall.
    ///
    /// # Arguments
    /// * `pos` - The room position.
    /// * `wall` - The wall to open.
    fn open(&mut self, pos: Pos, wall: &'static wall::Wall) {
        self.set_open(pos, wall, true);
    }

    /// Closes a wall.
    ///
    /// # Arguments
    /// * `pos` - The room position.
    /// * `wall` - The wall to close.
    fn close(&mut self, pos: Pos, wall: &'static wall::Wall) {
        self.set_open(pos, wall, false);
    }

    /// Returns the back of a wall.
    ///
    /// The back is the other side of the wall, located in a neighbouring room.
    ///
    /// # Arguments
    /// * `pos` - The room position.
    /// * `wall` - The wall.
    fn back(&self,
            pos: Pos,
            wall: &'static wall::Wall)
            -> (Pos, &'static wall::Wall) {
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
    fn opposite(&self,
                pos: Pos,
                wall: &'static wall::Wall)
                -> Option<&'static wall::Wall>;

    /// Returns all walls of a specific room.
    ///
    /// # Arguments
    /// * `pos` - The room position.
    fn walls(&self, pos: Pos) -> &[&'static wall::Wall];
}

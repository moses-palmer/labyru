extern crate ndarray;

pub mod room;

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
trait Maze<T>: Rooms<T> where T: Clone + Default {}

pub mod room;

extern crate ndarray;

use ndarray::Array2;

use room::Room;


/// A maze is a collection of rooms.
pub struct Maze<R>
    where R: Room
{
    /// The actual room container
    rooms: Array2<R>,
}


#[cfg(test)]
mod tests;

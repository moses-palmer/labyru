pub mod room;

extern crate ndarray;

use ndarray::{Array2, Axis};

use room::Room;


/// A maze is a collection of rooms.
pub struct Maze<R>
    where R: Room
{
    /// The actual room container
    rooms: Array2<R>,
}


impl<R> Maze<R>
    where R: Room
{
    /// Creates a new maze with all rooms closed.
    ///
    /// # Arguments
    /// * `width` - The width of the maze.
    /// * `height` - The height of the maze.
    pub fn new(width: usize, height: usize) -> Maze<R> {
        Maze { rooms: Array2::from_elem((width, height), R::default()) }
    }

    /// The number of rooms across the maze, horizontally.
    pub fn width(&self) -> usize {
        self.rooms.len_of(Axis(0))
    }

    /// The number of rooms across the maze, vertically.
    pub fn height(&self) -> usize {
        self.rooms.len_of(Axis(1))
    }
}


#[cfg(test)]
mod tests;

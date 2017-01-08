/// A room position.
///
/// The position is not an attribute of a [room](trait.Room.html), but a room
/// can be accessed from a [maze](../struct.Maze.html).
pub type Pos = (usize, usize);


/// A room is a part of a maze.
///
/// It has walls and openings connecting it with other rooms.
pub trait Room: Clone + Default {}

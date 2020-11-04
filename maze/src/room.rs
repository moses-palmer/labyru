#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::wall;

/// A room is a part of a maze.
///
/// It has walls, openings connecting it with other rooms, and asssociated data.
///
/// It does not know its location.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Room<T>
where
    T: Clone,
{
    walls: wall::Mask,

    /// Whether this room has been visited. This is true if at least one door
    /// has at any time been opened.
    pub visited: bool,

    /// The data associated with this room.
    pub data: T,
}

impl<T> Default for Room<T>
where
    T: Clone + Default,
{
    fn default() -> Self {
        Self {
            walls: wall::Mask::default(),
            visited: false,
            data: T::default(),
        }
    }
}

impl<T> Room<T>
where
    T: Clone,
{
    /// Whether a specified wall is open.
    ///
    /// # Example
    ///
    /// ```
    /// # use maze::room::*;
    /// # let mut room: Room<_> = false.into();
    /// # let wall = &maze::shape::quad::walls::LEFT;
    ///
    /// room.set_open(wall, true);
    /// assert!(room.is_open(wall));
    /// ```
    ///
    /// # Arguments
    /// *  `wall` - The wall to check.
    pub fn is_open(&self, wall: &'static wall::Wall) -> bool {
        self.walls & wall.mask() != 0
    }

    /// Sets whether a wall is open.
    ///
    /// # Arguments
    /// *  `wall` - The wall to set.
    /// *  `state` - Whether the wall is open.
    pub fn set_open(&mut self, wall: &'static wall::Wall, value: bool) {
        if value {
            self.open(wall);
        } else {
            self.close(wall);
        }
    }

    /// Opens a wall.
    ///
    /// # Arguments
    /// *  `wall` - The wall to open.
    pub fn open(&mut self, wall: &'static wall::Wall) {
        self.walls |= wall.mask();
        self.visited = true;
    }

    /// Closes a wall.
    ///
    /// # Arguments
    /// *  `wall` - The wall to close.
    pub fn close(&mut self, wall: &'static wall::Wall) {
        self.walls &= !wall.mask();
    }

    /// Returns the number of open walls.
    pub fn open_walls(&self) -> usize {
        self.walls.count_ones() as usize
    }

    /// Creates a copy of this room with new data.
    ///
    /// # Arguments
    /// *  `data` - The new data.
    pub fn with_data<U>(&self, data: U) -> Room<U>
    where
        U: Clone,
    {
        Room {
            walls: self.walls,
            visited: self.visited,
            data,
        }
    }
}

impl<T> From<T> for Room<T>
where
    T: Clone,
{
    /// Constructs a non-visited room with data.
    ///
    /// # Arguments
    /// *  `source` - The data content.
    fn from(source: T) -> Self {
        Self {
            walls: 0,
            visited: false,
            data: source,
        }
    }
}

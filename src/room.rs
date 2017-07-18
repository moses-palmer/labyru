use matrix;
use wall;


/// A room is a part of a maze.
///
/// It has walls and openings connecting it with other rooms and a data content.
#[derive(Clone, Copy, Debug, Default)]
pub struct Room {
    walls: wall::Mask,
}


impl Room {
    /// Returns whether a specified wall is open.
    ///
    /// # Arguments
    /// `wall` - The wall to check.
    pub fn is_open(&self, wall: &'static wall::Wall) -> bool {
        self.walls & wall.mask() != 0
    }

    /// Sets whether a wall is open..
    ///
    /// # Arguments
    /// `wall` - The wall to set.
    // `state` - Whether the wall is open.
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
    /// `wall` - The wall to open.
    pub fn open(&mut self, wall: &'static wall::Wall) {
        self.walls |= wall.mask();
    }

    /// Closes a wall.
    ///
    /// # Arguments
    /// `wall` - The wall to close.
    pub fn close(&mut self, wall: &'static wall::Wall) {
        self.walls &= !wall.mask();
    }
}


pub type Rooms = matrix::Matrix<Room>;

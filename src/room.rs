use wall;


/// A room is a part of a maze.
///
/// It has walls and openings connecting it with other rooms and a data content.
#[derive(Clone, Debug, Default)]
pub struct Room<T>
    where T: Clone + Default
{
    walls: wall::Mask,
    data: T,
}


impl<T> Room<T>
    where T: Clone + Default
{
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

    /// Retrieves a reference to the room data.
    pub fn data(&self) -> &T {
        &self.data
    }

    /// Retrieves a mutable reference to the room data.
    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

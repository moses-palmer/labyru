/// A room is a part of a maze.
///
/// It has walls and openings connecting it with other rooms and a data content.
#[derive(Clone, Debug, Default)]
pub struct Room<T>
    where T: Clone + Default
{
    data: T,
}


impl<T> Room<T>
    where T: Clone + Default
{
    /// Retrieves a reference to the room data.
    pub fn data(&self) -> &T {
        &self.data
    }

    /// Retrieves a mutable reference to the room data.
    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

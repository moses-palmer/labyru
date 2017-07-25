use matrix;


/// A physical position.
pub type Pos = (f32, f32);


/// An object that has some "physical" properties.
pub trait Physical {
    /// Returns the "physical" centre of a matrix position.
    ///
    /// # Arguments
    /// * `pos` - The matrix position.
    fn center(&self, pos: matrix::Pos) -> Pos;
}

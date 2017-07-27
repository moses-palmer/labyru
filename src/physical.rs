use matrix;

use WallPos;


/// A physical position.
pub type Pos = (f32, f32);


/// An object that has some "physical" properties.
pub trait Physical {
    /// Returns the "physical" centre of a matrix position.
    ///
    /// # Arguments
    /// * `pos` - The matrix position.
    fn center(&self, pos: matrix::Pos) -> Pos;

    /// Returns the "physical" positions of the two corners of a wall.
    ///
    /// # Arguments
    /// * `pos` - The matrix position.
    /// * `wall` - The wall.
    fn corners(&self, wall_pos: WallPos) -> (Pos, Pos) {
        let center = self.center(wall_pos.0);
        (
            (
                center.0 + wall_pos.1.span.0.cos(),
                center.1 + wall_pos.1.span.0.sin(),
            ),
            (
                center.0 + wall_pos.1.span.1.cos(),
                center.1 + wall_pos.1.span.1.sin(),
            ),
        )
    }
}

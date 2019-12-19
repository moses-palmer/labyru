use crate::Maze;

impl<T> Maze<T>
where
    T: Clone + Default,
{
    /// Calculates the _view box_ for an object when rendered.
    ///
    /// The returned tuple _(left, top, width, height)_ is the minimal rectangle
    /// that will contain the walls of the maze.
    pub fn viewbox(&self) -> (f32, f32, f32, f32) {
        let view_box = self.shape().viewbox(self.width(), self.height());
        (
            view_box.corner.x,
            view_box.corner.y,
            view_box.width,
            view_box.height,
        )
    }
}

#[cfg(feature = "render-svg")]
pub mod svg;

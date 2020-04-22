use crate::physical;
use crate::Maze;

impl<T> Maze<T>
where
    T: Clone + Default,
{
    /// Calculates the _view box_ for an object when rendered.
    ///
    /// The returned value is the minimal rectangle that will contain this
    /// maze.
    pub fn viewbox(&self) -> physical::ViewBox {
        self.shape().viewbox(self.width(), self.height())
    }
}

#[cfg(feature = "render-svg")]
pub mod svg;

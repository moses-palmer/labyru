use std::ops;

use maze;
use maze::matrix;
use maze::physical;

/// Translates physical positions to facets.
pub trait Facet {
    /// Translates a physical position to a matrix cell.
    ///
    /// # Arguments
    /// *  `pos` - The physical position to translate.
    fn facet(&self, pos: physical::Pos) -> matrix::Pos;

    /// The width of this facet.
    fn width(&self) -> usize;

    /// The height og this facet.
    fn height(&self) -> usize;
}

impl<T> Facet for maze::Maze<T>
where
    T: Clone + Copy + Default,
{
    fn facet(&self, pos: physical::Pos) -> matrix::Pos {
        self.room_at(pos)
    }

    fn width(&self) -> usize {
        maze::Maze::width(self)
    }

    fn height(&self) -> usize {
        maze::Maze::height(self)
    }
}

/// Focuses values into matrix cells defined by a facet.
pub trait Focus<F, T, U>
where
    F: Facet,
    T: Copy + Default,
    U: Copy + Default + ops::Add + ops::Div<usize, Output = T>,
{
    /// Passes values through a facet and collects their average in a matrix.
    ///
    /// # Arguments
    /// *  `facet` - The facet used to translate physical coordinates to matrix
    ///    coordinates.
    fn focus(self, facet: &F) -> matrix::Matrix<T>;
}

impl<'a, F, I, T, U> Focus<F, T, U> for &'a mut I
where
    F: Facet,
    I: Iterator<Item = (physical::Pos, U)>,
    T: Copy + Default,
    U: Copy + Default + ops::Add<U, Output = U> + ops::Div<usize, Output = T>,
{
    fn focus(self, facet: &F) -> matrix::Matrix<T> {
        self.fold(
            matrix::Matrix::<(usize, U)>::new(facet.width(), facet.height()),
            |mut acc, (physical_pos, value)| {
                let matrix_pos = facet.facet(physical_pos);
                if let Some((count, previous)) = acc.get(matrix_pos) {
                    acc[matrix_pos] = (count + 1, *previous + value);
                }
                acc
            },
        )
        .map(|(count, value)| value / count)
    }
}

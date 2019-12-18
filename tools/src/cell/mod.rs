use std::ops;

use maze;
use maze::matrix;
use maze::physical;

/// Translates physical positions to cells.
pub trait Cells {
    /// Translates a physical position to a matrix cell.
    ///
    /// # Arguments
    /// *  `pos` - The physical position to translate.
    fn cell(&self, pos: physical::Pos) -> matrix::Pos;

    /// The width of this facet.
    fn width(&self) -> usize;

    /// The height og this facet.
    fn height(&self) -> usize;
}

impl<T> Cells for maze::Maze<T>
where
    T: Clone + Copy + Default,
{
    fn cell(&self, pos: physical::Pos) -> matrix::Pos {
        self.room_at(pos)
    }

    fn width(&self) -> usize {
        maze::Maze::width(self)
    }

    fn height(&self) -> usize {
        maze::Maze::height(self)
    }
}

/// Splits values into matrix cells.
pub trait Splitter<C, T, U>
where
    C: Cells,
    T: Copy + Default,
    U: Copy + Default + ops::Add + ops::Div<usize, Output = T>,
{
    /// Passes values through cells and collects their average in a matrix.
    ///
    /// # Arguments
    /// *  `cells` - The cells used to translate physical coordinates to matrix
    ///    coordinates.
    fn split_by(self, cells: &C) -> matrix::Matrix<T>;
}

impl<'a, C, I, T, U> Splitter<C, T, U> for &'a mut I
where
    C: Cells,
    I: Iterator<Item = (physical::Pos, U)>,
    T: Copy + Default,
    U: Copy + Default + ops::Add<U, Output = U> + ops::Div<usize, Output = T>,
{
    fn split_by(self, cells: &C) -> matrix::Matrix<T> {
        self.fold(
            matrix::Matrix::<(usize, U)>::new(cells.width(), cells.height()),
            |mut acc, (physical_pos, value)| {
                let matrix_pos = cells.cell(physical_pos);
                if let Some((count, previous)) = acc.get(matrix_pos) {
                    acc[matrix_pos] = (count + 1, *previous + value);
                }
                acc
            },
        )
        .map(|(count, value)| *value / *count)
    }
}

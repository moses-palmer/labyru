use std::ops;

use maze::matrix;
use maze::physical;

/// Translates physical positions to cells.
pub trait Cells {
    /// Translates a physical position to a matrix cell.
    ///
    /// # Arguments
    /// *  `pos` - The physical position to translate.
    fn cell(&self, pos: physical::Pos) -> matrix::Pos;
}

impl Cells for maze::Shape {
    fn cell(&self, pos: physical::Pos) -> matrix::Pos {
        self.physical_to_cell(pos)
    }
}

/// Splits values into matrix cells.
pub trait Splitter<C, T, U>
where
    C: Cells,
    T: Copy,
    U: Copy + ops::Add + ops::Div<usize, Output = T>,
{
    /// Passes values through cells and collects their average in a matrix.
    ///
    /// # Arguments
    /// *  `cells` - The cells used to translate physical coordinates to matrix
    ///    coordinates.
    /// *  `width` - The expected width of the resulting matrix.
    /// *  `height` - The expected height of the resulting matrix.
    fn split_by(
        self,
        cells: &C,
        width: usize,
        height: usize,
    ) -> matrix::Matrix<T>;
}

impl<'a, C, I, T, U> Splitter<C, T, U> for &'a mut I
where
    C: Cells,
    I: Iterator<Item = (physical::Pos, U)>,
    T: Copy,
    U: Copy + Default + ops::Add<U, Output = U> + ops::Div<usize, Output = T>,
{
    fn split_by(
        self,
        cells: &C,
        width: usize,
        height: usize,
    ) -> matrix::Matrix<T> {
        self.fold(
            matrix::Matrix::<(usize, U)>::new(width, height),
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

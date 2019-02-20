use std;

/// A matrix position.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos {
    /// The column index.
    pub col: isize,

    /// The row index.
    pub row: isize,
}

/// A matrix is a two dimensional array.
///
/// Every cell has a value, which is addressed using a [Pos](type.Pos.html).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Matrix<T>
where
    T: Clone + Copy + Default,
{
    /// The width of the matrix.
    pub width: usize,

    /// The height of the matrix.
    pub height: usize,

    data: Vec<T>,
}

/// A matrix of rooms.
///
/// A room matrix has a width and a height, and rooms can be addressed by
/// position.
impl<T> Matrix<T>
where
    T: Clone + Copy + Default,
{
    /// Creates a new matrix with the specified dimensions.
    ///
    /// # Arguments
    /// * `width` - The width of the matrix.
    /// * `height` - The height of the matrix.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![T::default(); width * height],
        }
    }

    /// Determines whether a position is inside of the matrix.
    ///
    /// # Arguments
    /// * `pos` - The matrix position.
    pub fn is_inside(&self, pos: Pos) -> bool {
        pos.col >= 0
            && pos.row >= 0
            && pos.col < self.width as isize
            && pos.row < self.height as isize
    }

    /// Retrieves a reference to the value at a specific position if it exists.
    ///
    /// # Arguments
    /// * `pos` - The matrix position.
    pub fn get(&self, pos: Pos) -> Option<&T> {
        if self.is_inside(pos) {
            Some(&self.data[(pos.col + pos.row * self.width as isize) as usize])
        } else {
            None
        }
    }

    /// Retrieves a mutable reference to the value at a specific position if it
    /// exists.
    ///
    /// # Arguments
    /// * `pos` - The matrix position.
    pub fn get_mut(&mut self, pos: Pos) -> Option<&mut T> {
        if self.is_inside(pos) {
            Some(
                &mut self.data
                    [(pos.col + pos.row * self.width as isize) as usize],
            )
        } else {
            None
        }
    }

    /// Returns an iterator over all cell positions.
    ///
    /// The positions are returned row by row, starting from `(0, 0)` and ending
    /// with `(self.width - 1, self.height - 1)`.
    pub fn positions(&self) -> impl Iterator<Item = Pos> {
        PosIterator::new(self.width, self.height)
    }

    /// Returns an iterator over all cell values.
    ///
    /// The values are returned row by row, starting from `(0, 0)` and ending
    /// with `(self.width - 1, self.height - 1)`.
    pub fn values(&self) -> ValueIterator<'_, T> {
        ValueIterator::new(self)
    }

    /// Applies a mapping to this matrix.
    ///
    /// The return value is a matrix with the same dimensions as this one, but
    /// with every value mapped through the mapper.
    ///
    /// # Arguments
    /// * `mapper` - The mapping function.
    pub fn map<F, S>(&self, mapper: F) -> Matrix<S>
    where
        F: Fn(T) -> S,
        S: Clone + Copy + Default,
    {
        self.positions().fold(
            Matrix::new(self.width, self.height),
            |mut matrix, pos| {
                matrix[pos] = mapper(self[pos]);
                matrix
            },
        )
    }
}

pub trait AddableMatrix<T> {
    /// Adds another matrix to this one.
    ///
    /// If the matrices are of different dimensions, only the overlapping parts
    /// will be added.
    ///
    /// # Arguments
    /// * `other` - The matrix to add.
    fn add(self, other: Self) -> Self;
}

impl<T> AddableMatrix<T> for Matrix<T>
where
    T: std::ops::AddAssign + Clone + Copy + Default,
{
    fn add(mut self, other: Self) -> Self {
        let width = std::cmp::min(self.width, other.width);
        let height = std::cmp::min(self.height, other.height);
        for row in 0..height {
            for col in 0..width {
                let pos = Pos {
                    col: col as isize,
                    row: row as isize,
                };
                self[pos] += other[pos]
            }
        }

        self
    }
}

/// An iterator over matrix positions.
#[derive(Clone)]
pub struct PosIterator {
    /// The width of the matrix being iterated.
    width: usize,

    /// The height of the matrix being iterated.
    height: usize,

    /// The current position.
    current: isize,
}

impl PosIterator {
    /// Creates a new position iterator.
    ///
    /// # Arguments
    /// * `width` - The width of the matrix.
    /// * `height` - The height of the matrix.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            current: -1,
        }
    }
}

impl Iterator for PosIterator {
    type Item = Pos;

    /// Iterates over all cell positions in a matrix, row by row.
    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        if self.current >= (self.width * self.height) as isize {
            None
        } else {
            Some(Pos {
                col: self.current % self.width as isize,
                row: self.current / self.width as isize,
            })
        }
    }
}

/// An iterator over matrix values.
pub struct ValueIterator<'a, T>
where
    T: 'a + Clone + Copy + Default,
{
    /// An iterator over positions
    pos_iter: PosIterator,

    /// The current position.
    matrix: &'a Matrix<T>,
}

impl<'a, T> ValueIterator<'a, T>
where
    T: 'a + Clone + Copy + Default,
{
    /// Creates a new position iterator.
    ///
    /// # Arguments
    /// * `matrix` - The matrix.
    pub fn new(matrix: &'a Matrix<T>) -> Self {
        Self {
            matrix,
            pos_iter: PosIterator::new(matrix.width, matrix.height),
        }
    }
}

impl<'a, T> Iterator for ValueIterator<'a, T>
where
    T: 'a + Clone + Copy + Default,
{
    type Item = T;

    /// Iterates over all cell values in a matrix, row by row.
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pos) = self.pos_iter.next() {
            Some(self.matrix[pos])
        } else {
            None
        }
    }
}

impl<T> std::ops::Index<Pos> for Matrix<T>
where
    T: Clone + Copy + Default,
{
    type Output = T;

    /// Retrieves a reference to the value at a specific position.
    ///
    /// # Arguments
    /// * `pos` - The matrix position.
    ///
    /// # Panics
    /// Accessing a cell where [is_inside](#method.is_inside) returns `false`
    /// will cause a panic. Use [get](#method.get) to avoid this.
    fn index(&self, pos: Pos) -> &Self::Output {
        if self.is_inside(pos) {
            &self.data[(pos.col + pos.row * self.width as isize) as usize]
        } else {
            panic!()
        }
    }
}

impl<T> std::ops::IndexMut<Pos> for Matrix<T>
where
    T: Clone + Copy + Default,
{
    /// Retrieves a mutable reference to the value at a specific position.
    ///
    /// # Arguments
    /// * `pos` - The matrix position.
    ///
    /// # Panics
    /// Accessing a cell where [is_inside](#method.is_inside) returns `false`
    /// will cause a panic. Use [get_mut](#method.get_mut) to avoid this.
    fn index_mut(&mut self, pos: Pos) -> &mut T {
        if self.is_inside(pos) {
            &mut self.data[(pos.col + pos.row * self.width as isize) as usize]
        } else {
            panic!()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn iterate_positions() {
        assert_eq!(
            vec![
                matrix_pos(0, 0),
                matrix_pos(1, 0),
                matrix_pos(2, 0),
                matrix_pos(0, 1),
                matrix_pos(1, 1),
                matrix_pos(2, 1)
            ],
            Matrix::<bool>::new(3, 2).positions().collect::<Vec<_>>()
        );
    }

    #[test]
    fn iterate_values() {
        let mut matrix = Matrix::<u8>::new(2, 2);
        matrix[matrix_pos(0, 0)] = 1;
        matrix[matrix_pos(1, 0)] = 2;
        matrix[matrix_pos(0, 1)] = 3;
        matrix[matrix_pos(1, 1)] = 4;
        assert_eq!(vec![1, 2, 3, 4], matrix.values().collect::<Vec<_>>());
    }

    #[test]
    fn map() {
        let mut matrix = Matrix::<u8>::new(2, 2);
        matrix[matrix_pos(0, 0)] = 1;
        matrix[matrix_pos(1, 0)] = 2;
        matrix[matrix_pos(0, 1)] = 3;
        matrix[matrix_pos(1, 1)] = 4;
        assert_eq!(
            vec![2, 3, 4, 5],
            matrix.map(|v| v + 1).values().collect::<Vec<_>>()
        );
    }

    #[test]
    fn eq() {
        let mut matrix1 = Matrix::<bool>::new(2, 2);
        matrix1[matrix_pos(1, 1)] = true;
        let mut matrix2 = Matrix::<bool>::new(2, 2);
        matrix2[matrix_pos(1, 1)] = true;

        assert_eq!(matrix1, matrix2);

        matrix2[matrix_pos(0, 0)] = true;
        assert!(matrix1 != matrix2);
    }
}

use std;


/// A matrix position.
pub type Pos = (isize, isize);


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
            width: width,
            height: height,
            data: vec![T::default(); width * height],
        }
    }

    /// Determines whether a position is inside of the matrix.
    ///
    /// # Arguments
    /// * `pos` - The matrix position.
    pub fn is_inside(&self, pos: Pos) -> bool {
        pos.0 >= 0 && pos.1 >= 0 && pos.0 < self.width as isize &&
            pos.1 < self.height as isize
    }


    /// Retrieves a reference to the value at a specific position if it exists.
    ///
    /// # Arguments
    /// * `pos` - The matrix position.
    pub fn get(&self, pos: Pos) -> Option<&T> {
        if self.is_inside(pos) {
            Some(&self.data[(pos.0 + pos.1 * self.width as isize) as usize])
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
                &mut self.data[(pos.0 + pos.1 * self.width as isize) as usize],
            )
        } else {
            None
        }
    }

    /// Returns an iterator over all cell positions.
    ///
    /// The positions are returned row by row, starting from `(0, 0)` and ending
    /// with `(self.width - 1, self.height - 1)`.
    pub fn positions(&self) -> PosIterator {
        PosIterator::new(self.width, self.height)
    }
}


/// An iterator over matrix positions.
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
            width: width,
            height: height,
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
            Some((
                self.current % self.width as isize,
                self.current / self.width as isize,
            ))
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
            &self.data[(pos.0 + pos.1 * self.width as isize) as usize]
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
            &mut self.data[(pos.0 + pos.1 * self.width as isize) as usize]
        } else {
            panic!()
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iterate() {
        assert_eq!(
            vec![(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1)],
            Matrix::<bool>::new(3, 2).positions().collect::<Vec<_>>()
        );
    }

    #[test]
    fn eq() {
        let mut matrix1 = Matrix::<bool>::new(2, 2);
        matrix1[(1, 1)] = true;
        let mut matrix2 = Matrix::<bool>::new(2, 2);
        matrix2[(1, 1)] = true;

        assert_eq!(matrix1, matrix2);

        matrix2[(0, 0)] = true;
        assert!(matrix1 != matrix2);
    }
}

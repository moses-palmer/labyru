//! # The data matrix
//!
//! A matrix is a two-dimensional array of data. A maze is a matrix of rooms.

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// A matrix position.
///
/// The coordinates of this type are signed, but valid matrix positions never
/// have negative coordinates.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
pub struct Pos {
    /// The column index.
    ///
    /// Valid values are always zero or greater.
    pub col: isize,

    /// The row index.
    ///
    /// Valid values are always zero or greater.
    pub row: isize,
}

impl<T> From<(T, T)> for Pos
where
    T: Into<isize>,
{
    /// Converts the tuple _(x, y)_ to `Pos { x, y }`.
    ///
    /// # Example
    ///
    /// ```
    /// # use maze::matrix::Pos;
    ///
    /// assert_eq!(
    ///     Pos::from((1i16,    2i16)),
    ///     Pos { col: 1,  row: 2 },
    /// );
    /// assert_eq!(
    ///     Pos::from((1i8,     2i8)),
    ///     Pos { col: 1,  row: 2 },
    /// );
    /// ```
    fn from((col, row): (T, T)) -> Self {
        Pos {
            col: col.into(),
            row: row.into(),
        }
    }
}

/// A matrix is a two dimensional array.
///
/// Every cell has a value, which is addressed using a [`Pos`].
///
/// [`Pos`]: struct.Pos.html
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Matrix<T>
where
    T: Clone,
{
    /// The width of the matrix.
    pub width: usize,

    /// The height of the matrix.
    pub height: usize,

    data: Vec<T>,
}

impl<T> Matrix<T>
where
    T: Clone + Default,
{
    /// Creates a new matrix with the specified dimensions.
    ///
    /// Every cell is initiated with the `default` value of the type.
    ///
    /// # Arguments
    /// *  `width` - The width of the matrix.
    /// *  `height` - The height of the matrix.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![T::default(); width * height],
        }
    }
}

impl<T> Matrix<T>
where
    T: Clone,
{
    /// Constructs an initialised matrix.
    ///
    /// This constructor can be used when no default value exists for the data
    /// type.
    ///
    /// # Example
    ///
    /// ```
    /// # use maze::matrix::*;
    ///
    /// let matrix = Matrix::new_with_data(
    ///     2,
    ///     2,
    ///     |pos| (pos.col + 1) * (pos.row + 1),
    /// );
    /// assert_eq!(
    ///     matrix.values().cloned().collect::<Vec<_>>(),
    ///     vec![
    ///         1,
    ///         2,
    ///         2,
    ///         4,
    ///     ],
    /// );
    ///
    /// ```
    pub fn new_with_data<F>(width: usize, height: usize, data: F) -> Self
    where
        F: FnMut(Pos) -> T,
    {
        Self {
            width,
            height,
            data: PosIterator::new(width, height).map(data).collect(),
        }
    }

    /// Applies a mapping to this matrix.
    ///
    /// The return value is a matrix with the same dimensions as this one, but
    /// with every value mapped through the mapper.
    ///
    /// # Example
    ///
    /// ```
    /// # use maze::matrix::*;
    /// # type Matrix = maze::matrix::Matrix<u32>;
    ///
    /// let mut matrix = Matrix::new(2, 2);
    /// matrix[Pos { col: 0, row: 0 }] = 0;
    /// matrix[Pos { col: 1, row: 0 }] = 1;
    /// matrix[Pos { col: 0, row: 1 }] = 2;
    /// matrix[Pos { col: 1, row: 1 }] = 3;
    /// assert_eq!(
    ///     matrix.map(|v| v + 1).values().cloned().collect::<Vec<_>>(),
    ///     vec![
    ///         1,
    ///         2,
    ///         3,
    ///         4,
    ///     ],
    /// );
    /// ```
    ///
    /// # Arguments
    /// *  `mapper` - The mapping function.
    ///
    pub fn map<F, S>(&self, mut mapper: F) -> Matrix<S>
    where
        F: FnMut(&T) -> S,
        S: Clone,
    {
        Matrix::new_with_data(self.width, self.height, |pos| mapper(&self[pos]))
    }

    /// Applies a mapping to this matrix.
    ///
    /// The return value is a matrix with the same dimensions as this one, but
    /// with every value mapped through the mapper.
    ///
    /// # Example
    ///
    /// ```
    /// # use maze::matrix::*;
    /// # type Matrix = maze::matrix::Matrix<u32>;
    ///
    /// let mut matrix = Matrix::new(2, 2);
    /// matrix[Pos { col: 0, row: 0 }] = 0;
    /// matrix[Pos { col: 1, row: 0 }] = 1;
    /// matrix[Pos { col: 0, row: 1 }] = 2;
    /// matrix[Pos { col: 1, row: 1 }] = 3;
    /// assert_eq!(
    ///     matrix.map_with_pos(|p, v| ((p.col, p.row), v + 1)).values()
    ///         .cloned()
    ///         .collect::<Vec<_>>(),
    ///     vec![
    ///         ((0, 0), 1),
    ///         ((1, 0), 2),
    ///         ((0, 1), 3),
    ///         ((1, 1), 4),
    ///     ],
    /// );
    /// ```
    ///
    /// # Arguments
    /// *  `mapper` - The mapping function.
    ///
    pub fn map_with_pos<F, S>(&self, mut mapper: F) -> Matrix<S>
    where
        F: FnMut(Pos, &T) -> S,
        S: Clone,
    {
        Matrix::new_with_data(self.width, self.height, |pos| {
            mapper(pos, &self[pos])
        })
    }

    /// Whether a position is inside of the matrix.
    ///
    /// # Example
    ///
    /// ```
    /// # use maze::matrix::*;
    /// # type Matrix = maze::matrix::Matrix<bool>;
    ///
    /// let matrix = Matrix::new(5, 5);
    /// assert!(matrix.is_inside(Pos {col: 0, row: 0 }));
    /// assert!(!matrix.is_inside(Pos {
    ///     col: matrix.width as isize,
    ///     row: matrix.height as isize,
    /// }));
    /// # assert!(!matrix.is_inside(Pos { col: -1, row: -1 }));
    /// # assert!(matrix.is_inside(Pos {
    /// #     col: matrix.width as isize - 1,
    /// #     row: matrix.height as isize - 1,
    /// # }));
    /// ```
    ///
    /// # Arguments
    /// *  `pos` - The matrix position.
    pub fn is_inside(&self, pos: Pos) -> bool {
        pos.col >= 0
            && pos.row >= 0
            && pos.col < self.width as isize
            && pos.row < self.height as isize
    }

    /// Retrieves a reference to the value at a specific position if it exists.
    ///
    /// # Example
    ///
    /// ```
    /// # use maze::matrix::*;
    /// # type Matrix = maze::matrix::Matrix<u32>;
    ///
    /// let mut matrix = Matrix::new(5, 5);
    /// matrix[Pos { col: 1, row: 1 }] = 5;
    /// assert_eq!(
    ///     matrix.get(Pos { col: 1, row: 1 }),
    ///     Some(&5),
    /// );
    /// # assert_eq!(
    /// #     matrix.get(Pos { col: -1, row: -1 }),
    /// #     None,
    /// # );
    /// ```
    ///
    /// # Arguments
    /// *  `pos` - The matrix position.
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
    /// # Example
    ///
    /// ```
    /// # use maze::matrix::*;
    /// # type Matrix = maze::matrix::Matrix<u32>;
    ///
    /// let mut matrix = Matrix::new(5, 5);
    /// *matrix.get_mut(Pos { col: 1, row: 1 }).unwrap() = 5;
    /// assert_eq!(
    ///     matrix[Pos { col: 1, row: 1 }],
    ///     5,
    /// );
    /// ```
    ///
    /// # Arguments
    /// *  `pos` - The matrix position.
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

    /// Iterates over all cell positions.
    ///
    /// The positions are visited row by row, starting with `(0, 0)` and ending
    /// with `(self.width - 1, self.height - 1)`.
    ///
    /// # Example
    ///
    /// ```
    /// # use maze::matrix::*;
    /// # type Matrix = maze::matrix::Matrix<u32>;
    ///
    /// let matrix = Matrix::new(2, 2);
    /// assert_eq!(
    ///     matrix.positions().collect::<Vec<_>>(),
    ///     vec![
    ///         Pos { col: 0, row: 0 },
    ///         Pos { col: 1, row: 0 },
    ///         Pos { col: 0, row: 1 },
    ///         Pos { col: 1, row: 1 },
    ///     ],
    /// );
    /// ```
    pub fn positions(&self) -> impl Iterator<Item = Pos> {
        PosIterator::new(self.width, self.height)
    }

    /// Iterates over all cell values.
    ///
    /// The values are visited row by row, starting with `(0, 0)` and ending
    /// with `(self.width - 1, self.height - 1)`.
    ///
    /// # Example
    ///
    /// ```
    /// # use maze::matrix::*;
    /// # type Matrix = maze::matrix::Matrix<u32>;
    ///
    /// let mut matrix = Matrix::new(2, 2);
    /// matrix[Pos { col: 0, row: 0 }] = 0;
    /// matrix[Pos { col: 1, row: 0 }] = 1;
    /// matrix[Pos { col: 0, row: 1 }] = 2;
    /// matrix[Pos { col: 1, row: 1 }] = 3;
    /// assert_eq!(
    ///     matrix.values().cloned().collect::<Vec<_>>(),
    ///     vec![
    ///         0,
    ///         1,
    ///         2,
    ///         3,
    ///     ],
    /// );
    /// ```
    pub fn values(&self) -> ValueIterator<'_, T> {
        ValueIterator::new(self)
    }
}

impl<T> Matrix<T>
where
    T: Copy + Eq + PartialEq + PartialOrd + Ord,
{
    /// All edges between areas with different values.
    ///
    /// The return value is a mapping from source area value and destination
    /// area value to a set of matrix positions with connections.
    ///
    /// For a uniform matrix, this method will return an empty set.
    ///
    /// # Example
    ///
    /// ```
    /// # use maze::matrix::*;
    /// # type Matrix = maze::matrix::Matrix<u32>;
    ///
    /// fn data(pos: Pos) -> u32 {
    ///     pos.col as u32
    /// }
    ///
    /// fn neighbors(pos: Pos) -> impl Iterator<Item = Pos> {
    ///     vec![
    ///         (pos.col - 1, pos.row).into(),
    ///         (pos.col, pos.row - 1).into(),
    ///         (pos.col, pos.row + 1).into(),
    ///         (pos.col + 1, pos.row).into(),
    ///     ].into_iter()
    /// }
    ///
    /// let mut matrix = Matrix::new_with_data(3, 2, data);
    /// assert_eq!(
    ///     matrix.edges(neighbors).iter()
    ///         .map(|(k, v)| (
    ///             k.clone(),
    ///             v.iter().cloned().collect::<Vec<_>>()),
    ///         )
    ///         .collect::<Vec<_>>(),
    ///     vec![
    ///         ((0, 1), vec![
    ///             ((0isize, 0isize).into(), (1isize, 0isize).into()),
    ///             ((0isize, 1isize).into(), (1isize, 1isize).into()),
    ///         ]),
    ///         ((1, 2), vec![
    ///             ((1isize, 0isize).into(), (2isize, 0isize).into()),
    ///             ((1isize, 1isize).into(), (2isize, 1isize).into()),
    ///         ]),
    ///     ],
    /// );
    /// ```
    ///
    /// # Arguments
    /// *  `neighbors` - A function returning neighbours to consider for each
    ///    cell.
    pub fn edges<F, I>(
        &self,
        neighbors: F,
    ) -> BTreeMap<(T, T), BTreeSet<(Pos, Pos)>>
    where
        F: Fn(Pos) -> I,
        I: Iterator<Item = Pos>,
    {
        self.positions().fold(BTreeMap::new(), |mut acc, p1| {
            neighbors(p1)
                .filter(|&p2| self.is_inside(p2))
                .flat_map(|p2| {
                    let k1 = self[p1];
                    let k2 = self[p2];
                    k1.partial_cmp(&k2).and_then(|val| match val {
                        Ordering::Less => Some(((k1, k2), (p1, p2))),
                        Ordering::Greater => Some(((k2, k1), (p2, p1))),
                        _ => None,
                    })
                })
                .for_each(|(k, v)| {
                    acc.entry(k).or_insert_with(BTreeSet::new).insert(v);
                });
            acc
        })
    }
}

impl<T> Matrix<T>
where
    T: Clone + Copy + PartialEq,
{
    /// Fills all rooms reachable from `pos` in `matrix` with the value
    /// `value`.
    ///
    /// Filling will start at `pos`, and `neighbors` will be used to find the
    /// next cells. Any cell with the value `value` is ignored; thus, if all
    /// neighbours of `pos` already have the value `value`, filling will stop
    /// immediately.
    ///
    /// If `pos` has the value `value`, however, filling may proceed with
    /// neighbours.
    ///
    /// The number of filled rooms is returned.
    ///
    /// # Arguments
    /// *  `pos` - The starting position.
    /// *  `value` - The value with which to fill.
    /// *  `neighbors``- A function returning neighbours given a matrix
    ///    position.
    pub fn fill<F, I>(&mut self, pos: Pos, value: T, neighbors: F) -> usize
    where
        F: Fn(Pos) -> I,
        I: Iterator<Item = Pos>,
    {
        // Cancel immediately if the position is outside of the matrix
        if !self.is_inside(pos) {
            return 0;
        }

        // Mark the initial room
        let mut result = 1;
        self[pos] = value;

        // Keep track of where we have been
        let mut path = vec![pos];

        // Traverse the rooms depth first
        while !path.is_empty() {
            let current = path[path.len() - 1];
            if let Some(next) = neighbors(current)
                .flat_map(|pos| {
                    self.get(pos).and_then(|&v| {
                        if v != value {
                            Some(pos)
                        } else {
                            None
                        }
                    })
                })
                .next()
            {
                result += 1;
                self[next] = value;
                path.push(next);
            } else {
                path.pop();
            }
        }

        result
    }
}

impl<T> std::ops::Add for Matrix<T>
where
    T: std::ops::AddAssign + Clone + Copy,
{
    type Output = Self;

    /// Adds another matrix to this one.
    ///
    /// If the matrices are of different dimensions, only the overlapping parts
    /// will be added.
    ///
    /// # Examples
    ///
    /// ```
    /// # use maze::matrix::*;
    /// # type Matrix = maze::matrix::Matrix<u32>;
    ///
    /// let mut matrix1 = Matrix::new(2, 2);
    /// matrix1[Pos { col: 0, row: 0 }] = 0;
    /// matrix1[Pos { col: 1, row: 0 }] = 1;
    /// matrix1[Pos { col: 0, row: 1 }] = 2;
    /// matrix1[Pos { col: 1, row: 1 }] = 3;
    /// let mut matrix2 = Matrix::new(2, 2);
    /// matrix2[Pos { col: 0, row: 0 }] = 5;
    /// matrix2[Pos { col: 1, row: 1 }] = 5;
    /// assert_eq!(
    ///     (matrix1 + matrix2).map(|v| v + 1)
    ///         .values()
    ///         .cloned()
    ///         .collect::<Vec<_>>(),
    ///     vec![
    ///         6,
    ///         2,
    ///         3,
    ///         9,
    ///     ],
    /// );
    /// ```
    ///
    /// # Arguments
    /// *  `other` - The matrix to add.
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
    /// *  `width` - The width of the matrix.
    /// *  `height` - The height of the matrix.
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
    T: 'a + Clone,
{
    /// An iterator over positions
    pos_iter: PosIterator,

    /// The current position.
    matrix: &'a Matrix<T>,
}

impl<'a, T> ValueIterator<'a, T>
where
    T: 'a + Clone,
{
    /// Creates a new position iterator.
    ///
    /// # Arguments
    /// *  `matrix` - The matrix.
    pub fn new(matrix: &'a Matrix<T>) -> Self {
        Self {
            matrix,
            pos_iter: PosIterator::new(matrix.width, matrix.height),
        }
    }
}

impl<'a, T> Iterator for ValueIterator<'a, T>
where
    T: 'a + Clone,
{
    type Item = &'a T;

    /// Iterates over all cell values in a matrix, row by row.
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pos) = self.pos_iter.next() {
            Some(&self.matrix[pos])
        } else {
            None
        }
    }
}

impl<T> std::ops::Index<Pos> for Matrix<T>
where
    T: Clone,
{
    type Output = T;

    /// Retrieves a reference to the value at a specific position.
    ///
    /// # Arguments
    /// *  `pos` - The matrix position.
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
    T: Clone,
{
    /// Retrieves a mutable reference to the value at a specific position.
    ///
    /// # Arguments
    /// *  `pos` - The matrix position.
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

/// Partitions a number into its integral part and a fraction.
///
/// Adding the fraction to the integral part will yield the original.
///
/// # Examples
///
/// ```
/// # use std::f32::EPSILON;
/// # use maze::matrix::*;
///
/// let (int, fract) = partition(1.2);
/// assert_eq!(int, 1);
/// assert!(
///     (fract - 0.2).abs() < EPSILON,
/// );
///
/// let (int, fract) = partition(-1.2);
/// assert_eq!(int, -2);
/// assert!(
///     (fract - 0.8).abs() < EPSILON,
/// );
/// ```
///
/// # Arguments
/// *  `x` - a number.
pub fn partition(x: f32) -> (isize, f32) {
    let index = x.floor() as isize;
    let rel = x.fract();
    (index, if x >= 0.0 { rel } else { rel + 1.0 })
}

/// Generates a matrix initialised with the value returned by a filter
/// function.
///
/// The return value contains the number of filtered rooms.
///
/// # Arguments
/// *  `width` - The width of the matrix to generate.
/// *  `height` - The height of the matrix to generate.
/// *  `filter` - A filter function.
pub fn filter<F>(
    width: usize,
    height: usize,
    filter: F,
) -> (usize, Matrix<bool>)
where
    F: Fn(Pos) -> bool,
{
    let mut result = Matrix::new(width, height);
    let count = result.positions().fold(0, |mut count, pos| {
        if filter(pos) {
            result[pos] = true;
            count += 1;
        }
        count
    });
    (count, result)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn pos_into() {
        let expected = Pos { col: 1, row: 2 };
        let actual: Pos = (1isize, 2isize).into();
        assert_eq!(expected, actual);
    }

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
        assert_eq!(
            vec![1, 2, 3, 4],
            matrix.values().cloned().collect::<Vec<_>>()
        );
    }

    #[test]
    fn edges_none() {
        let matrix = Matrix::<u8>::new(3, 3);
        assert_eq!(BTreeMap::new(), matrix.edges(all_neighbors));
    }

    #[test]
    fn edges_simple() {
        let matrix =
            Matrix::<u8>::new_with_data(3, 3, |pos| match pos.col % 3 {
                0 | 1 => 1,
                _ => 2,
            });

        assert_eq!(
            [(
                (1, 2),
                &[
                    ((1isize, 0isize), (2isize, 0isize)),
                    ((1isize, 1isize), (2isize, 1isize)),
                    ((1isize, 2isize), (2isize, 2isize))
                ]
            )]
            .iter()
            .map(|(areas, positions)| (
                areas.clone(),
                positions
                    .iter()
                    .cloned()
                    .map(|(p1, p2)| (p1.into(), p2.into()))
                    .collect::<BTreeSet<_>>(),
            ))
            .collect::<BTreeMap<_, _>>(),
            matrix.edges(all_neighbors),
        );
    }

    #[test]
    fn edges_many() {
        let matrix =
            Matrix::<u8>::new_with_data(3, 3, |pos| match pos.col % 3 {
                0 => 1,
                1 => 2,
                _ => 3,
            });

        assert_eq!(
            [
                (
                    (1, 2),
                    &[
                        ((0isize, 0isize), (1isize, 0isize)),
                        ((0isize, 1isize), (1isize, 1isize)),
                        ((0isize, 2isize), (1isize, 2isize))
                    ]
                ),
                (
                    (2, 3),
                    &[
                        ((1isize, 0isize), (2isize, 0isize)),
                        ((1isize, 1isize), (2isize, 1isize)),
                        ((1isize, 2isize), (2isize, 2isize))
                    ]
                )
            ]
            .iter()
            .map(|(areas, positions)| (
                areas.clone(),
                positions
                    .iter()
                    .cloned()
                    .map(|(p1, p2)| (p1.into(), p2.into()))
                    .collect::<BTreeSet<_>>(),
            ))
            .collect::<BTreeMap<_, _>>(),
            matrix.edges(all_neighbors),
        );
    }

    #[test]
    fn edges_nonuniform() {
        let matrix = Matrix::<u8>::new_with_data(5, 5, |pos| {
            if (pos.col - 3).abs() < 2 && (pos.row - 3).abs() < 2 {
                0
            } else {
                1
            }
        });

        assert_eq!(
            [(
                (0, 1),
                &[
                    ((2isize, 2isize), (1isize, 2isize)),
                    ((2isize, 2isize), (2isize, 1isize)),
                    ((2isize, 3isize), (1isize, 3isize)),
                    ((2isize, 4isize), (1isize, 4isize)),
                    ((3isize, 2isize), (3isize, 1isize)),
                    ((4isize, 2isize), (4isize, 1isize)),
                ]
            ),]
            .iter()
            .map(|(areas, positions)| (
                areas.clone(),
                positions
                    .iter()
                    .cloned()
                    .map(|(p1, p2)| (p1.into(), p2.into()))
                    .collect::<BTreeSet<_>>(),
            ))
            .collect::<BTreeMap<_, _>>(),
            matrix.edges(all_neighbors),
        );
    }

    #[test]
    fn filter_none() {
        let width = 5;
        let height = 5;
        let (count, matrix) = filter(width, height, |_| false);
        assert_eq!(0, count);
        assert!(matrix.values().all(|v| !v));
    }

    #[test]
    fn filter_all() {
        let width = 5;
        let height = 5;
        let (count, matrix) = filter(width, height, |_| true);
        assert_eq!(width * height, count);
        assert!(matrix.values().all(|&v| v));
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
            matrix.map(|&v| v + 1).values().cloned().collect::<Vec<_>>()
        );
    }

    #[test]
    fn eq() {
        fn data(pos: Pos) -> bool {
            pos == matrix_pos(1, 1)
        }
        let matrix1 = Matrix::new_with_data(2, 2, data);
        let mut matrix2 = Matrix::new_with_data(2, 2, data);

        assert_eq!(matrix1, matrix2);

        matrix2[matrix_pos(0, 0)] = !data(matrix_pos(0, 0));
        assert!(matrix1 != matrix2);
    }

    #[test]
    fn partition() {
        let (index, rel) = super::partition(1.2);
        assert_eq!(index, 1);
        assert!((rel - 0.2).abs() < 0.0001);

        let (index, rel) = super::partition(-1.2);
        assert_eq!(index, -2);
        assert!((rel - 0.8).abs() < 0.0001);
    }

    #[test]
    fn fill_closed() {
        let mut matrix = Matrix::new_with_data(10, 10, |pos| {
            if pos.col == 0 && pos.row == 0 {
                0
            } else {
                1
            }
        });
        let count = 1;
        let filled = matrix
            .fill(Pos { col: 0, row: 0 }.into(), 1, |_| [].iter().cloned());
        assert_eq!(count, filled);

        for pos in matrix.positions() {
            assert_eq!(1, matrix[pos]);
        }
    }

    #[test]
    fn fill_open() {
        let mut matrix = Matrix::new(10, 10);
        let count = matrix.width * matrix.height;
        let filled =
            matrix.fill(Pos { col: 0, row: 0 }.into(), 1, all_neighbors);
        assert_eq!(count, filled);

        for pos in matrix.positions() {
            assert_eq!(1, matrix[pos]);
        }
    }

    #[test]
    fn fill_semiopen() {
        let filter = |pos: Pos| pos.col >= pos.row;
        let mut matrix =
            Matrix::new_with_data(
                10,
                10,
                |pos| if filter(pos) { 0 } else { 1 },
            );
        let count = matrix.values().filter(|&&v| v == 0).count();
        let filled =
            matrix.fill(Pos { col: 0, row: 0 }.into(), 1, all_neighbors);
        assert_eq!(count, filled);

        for pos in matrix.positions() {
            assert_eq!(1, matrix[pos]);
        }
    }

    #[test]
    fn fill_separated() {
        let filter = |pos: Pos| pos.col < 2 || pos.col >= 8;
        let mut matrix =
            Matrix::new_with_data(
                10,
                10,
                |pos| if filter(pos) { 0 } else { 1 },
            );
        let count = matrix.height * 2;
        let filled =
            matrix.fill(Pos { col: 0, row: 0 }.into(), 1, all_neighbors);
        assert_eq!(count, filled);

        for pos in matrix.positions() {
            assert_eq!(
                if filter(pos) && pos.col >= 2 { 0 } else { 1 },
                matrix[pos],
            );
        }
    }

    /// Generates the positions of all neighbouring cells.
    ///
    /// # Arguments
    /// *  `pos` - The cell position for which to generate neighbours.
    fn all_neighbors(
        pos: Pos,
    ) -> impl Iterator<Item = Pos> + DoubleEndedIterator {
        vec![
            Pos {
                col: pos.col,
                row: pos.row - 1,
            },
            Pos {
                col: pos.col - 1,
                row: pos.row,
            },
            Pos {
                col: pos.col + 1,
                row: pos.row,
            },
            Pos {
                col: pos.col,
                row: pos.row + 1,
            },
        ]
        .into_iter()
    }
}

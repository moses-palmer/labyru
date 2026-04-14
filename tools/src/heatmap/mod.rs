use maze::{Maze, matrix};

/// A matrix of scores for rooms.
pub type HeatMap = matrix::Matrix<u32>;

/// Generates a heat map where the value for each cell is the number of times it has been traversed
/// when walking between the positions.
///
/// Any position pairs with no path between them will be ignored.
///
/// # Arguments
/// *  `positions` - The positions as the tuple `(from, to)`. These are used as
///    positions between which to walk.
pub fn generate<I, T>(maze: &Maze<T>, positions: I) -> HeatMap
where
    I: Iterator<Item = (matrix::Pos, matrix::Pos)>,
    T: Clone,
{
    let mut result = matrix::Matrix::new(maze.width(), maze.height());

    for (from, to) in positions {
        if let Some(path) = maze.walk(from, to) {
            for pos in path.into_iter() {
                result[pos] += 1;
            }
        }
    }

    result
}

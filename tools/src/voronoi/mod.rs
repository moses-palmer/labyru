use std::f32;

use maze;
use maze::matrix;
use maze::physical;

pub mod initialize;

/// A point description in the Voronoi diagram initialisation vector.
///
/// The first item is the physical location of this point and the second its
/// weight. The final item is the actual value.
pub type Point<V> = (physical::Pos, f32, V);

pub fn matrix<V, T>(
    maze: &maze::Maze<T>,
    points: Vec<Point<V>>,
) -> matrix::Matrix<V>
where
    V: Clone + Default,
    T: Clone,
{
    let mut result = matrix::Matrix::new(maze.width(), maze.height());

    // For each cell in the resulting matrix, retrieve the value of the point
    // closest to the centre of the room corresponding to the cell by iterating
    // over all points and selecting the one where the distance / weight ratio
    // is the smallest
    for pos in result.positions() {
        let center = maze.center(pos);
        if let Some(val) = points
            .iter()
            .map(|(p, w, val)| ((*p - center).value() / w, val))
            // We assume that that the weights are not exotic enough to cause
            // this to fail
            .min_by(|v1, v2| v1.0.partial_cmp(&v2.0).unwrap())
            .map(|(_, val)| val)
        {
            result[pos] = val.clone();
        }
    }

    result
}

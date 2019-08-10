use std::f32;

use maze;
use maze::matrix;
use maze::physical;

pub fn matrix<V>(
    maze: &maze::Maze,
    points: Vec<(physical::Pos, f32, V)>,
) -> matrix::Matrix<V>
where
    V: Clone + Copy + Default,
{
    let mut result = matrix::Matrix::new(maze.width(), maze.height());

    // For each cell in the resulting matrix, retrieve the value of the point
    // closest to the centre of the room corresponding to the cell
    for pos in result.positions() {
        let physical::Pos { x: cx, y: cy } = maze.center(pos);
        let (_, value) = points
            .iter()
            // Map points to a measurement of the distance to the current cell
            .map(|(physical::Pos { x, y }, w, v)| (x - cx, y - cy, w, v))
            .map(|(dx, dy, w, v)| ((dx * dx + dy * dy) / w, v))
            // Find the closest point
            .fold((f32::MAX, V::default()), |(ad, av), (cd, &cv)| {
                if cd < ad {
                    (cd, cv)
                } else {
                    (ad, av)
                }
            });
        result[pos] = value;
    }

    result
}

use crate::Maze;

use crate::matrix;

/// Initialises a maze by clearing all inner walls.
///
/// # Arguments
/// *  `maze``- The maze to initialise.
/// *  `_rng` - Not used.
/// *  `candidates` - A filter for the rooms to modify.
pub(crate) fn initialize<R, T>(
    mut maze: Maze<T>,
    _rng: &mut R,
    candidates: matrix::Matrix<bool>,
) -> Maze<T>
where
    R: super::Randomizer + Sized,
    T: Clone,
{
    for pos in maze.positions().filter(|&pos| candidates[pos]) {
        for wall in maze.walls(pos) {
            let (pos, wall) = maze.back((pos, wall));
            if *candidates.get(pos).unwrap_or(&false) {
                maze.open((pos, wall));
            }
        }
    }

    maze
}

use crate::{Maze, WallPos, matrix};

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
        for &wall in maze.walls(pos) {
            let wall_pos = WallPos { pos, wall }.back();
            if *candidates.get(wall_pos.pos).unwrap_or(&false) {
                maze.open(wall_pos);
            }
        }
    }

    maze
}

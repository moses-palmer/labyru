use crate::Maze;

use crate::matrix;

/// Initialises a maze by clearing all inner walls.
///
/// This method will ignore rooms for which `filter` returns `false`.
///
/// # Arguments
/// *  `_rng` - Not used.
/// *  `filter` - A predicate filtering rooms to consider.
pub(crate) fn initialize<F, R, T>(
    mut maze: Maze<T>,
    _rng: &mut R,
    filter: F,
) -> Maze<T>
where
    F: Fn(matrix::Pos) -> bool,
    R: super::Randomizer + Sized,
    T: Clone + Default,
{
    let (count, candidates) =
        matrix::filter(maze.width(), maze.height(), filter);
    if count == 0 {
        return maze;
    }

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

use std::collections::BTreeSet;

use crate::Maze;

use crate::matrix;

/// Initialises a maze using the _Braid_ algorithm.
///
/// This method will leave no dead ends in the final maze; all rooms will have
/// at least two open walls.
///
/// # Arguments
/// *  `maze``- The maze to initialise.
/// *  `rng` - A random number generator.
/// *  `candidates` - A filter for the rooms to modify.
pub(crate) fn initialize<R, T>(
    mut maze: Maze<T>,
    rng: &mut R,
    candidates: matrix::Matrix<bool>,
) -> Maze<T>
where
    R: super::Randomizer + Sized,
    T: Clone,
{
    // First remove all inner walls
    for pos in maze.positions().filter(|&pos| candidates[pos]) {
        for wall in maze.walls(pos) {
            let (pos, wall) = maze.back((pos, wall));
            if *candidates.get(pos).unwrap_or(&false) {
                maze.open((pos, wall));
            }
        }
    }

    // List all possible walls
    let walls = maze
        .positions()
        .filter(|&pos| candidates[pos])
        .flat_map(|pos| {
            maze.wall_positions(pos)
                .map(|wall_pos| (wall_pos, maze.back(wall_pos)))
        })
        .filter(|(_, back)| *candidates.get(back.0).unwrap_or(&false))
        .map(|(wall_pos, back)| {
            let dx = wall_pos.0.col - back.0.col;
            let dy = wall_pos.0.row - back.0.row;
            if dy < 0 || (dy == 0 && dx < 0) {
                wall_pos
            } else {
                back
            }
        })
        .collect::<BTreeSet<_>>();

    // Randomize the wall array
    let mut walls = walls.iter().collect::<Vec<_>>();
    let len = walls.len();
    for i in 0..len {
        walls.swap(i, rng.range(0, len));
    }

    // Attempt to add every wall, but make sure no dead-ends appear
    for &wall_pos in walls {
        let back = maze.back(wall_pos);
        if maze[wall_pos.0].open_walls() > 2 && maze[back.0].open_walls() > 2 {
            maze.close(wall_pos);
        }
    }

    super::connect_all(&mut maze, rng, |pos| {
        *candidates.get(pos).unwrap_or(&false)
    });

    maze
}

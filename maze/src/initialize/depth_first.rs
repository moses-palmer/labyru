use crate::Maze;

use crate::matrix;

/// Initialises a maze using the _Depth First_ algorithm.
///
/// See [here](https://en.wikipedia.org/wiki/Maze_generation_algorithm) for a
/// description of the algorithm.
///
/// The maze should be fully closed; any already open walls will be ignored and
/// kept.
///
/// This method will ignore rooms for which `filter` returns `false`.
///
/// # Arguments
/// *  `maze``- The maze to initialise.
/// *  `rng` - A random number generator.
/// *  `filter` - A predicate filtering rooms to consider.
pub(crate) fn initialize<F, R, T>(
    mut maze: Maze<T>,
    rng: &mut R,
    filter: F,
) -> Maze<T>
where
    F: Fn(matrix::Pos) -> bool,
    R: super::Randomizer + Sized,
    T: Clone,
{
    let (count, mut candidates) =
        matrix::filter(maze.width(), maze.height(), filter);
    if count == 0 {
        return maze;
    }

    // The backracking path is initially empty
    let mut path = Vec::new();

    // Start in a random room; we know that at least one candidate exists
    let mut current = super::random_room(rng, &candidates).unwrap();

    loop {
        candidates[current] = false;

        // Find all non-visited neighbours as the tuple (neighbour-position,
        // wall-from-current)
        let neighbors = maze
            .walls(current)
            .iter()
            .map(|wall| maze.back((current, wall)))
            .filter(|&(pos, _)| *candidates.get(pos).unwrap_or(&false))
            .map(|(pos, wall)| (pos, maze.back((pos, wall)).1))
            .collect::<Vec<_>>();

        // If any exists, move to a random one and update the path, otherwise
        // backtrack to  the previous room; since the maze may be segmented, we
        // must also attempt to find a new random room
        if !neighbors.is_empty() {
            let (next, wall) = neighbors[rng.range(0, neighbors.len())];
            maze.open((current, wall));
            path.push(current);
            current = next;
        } else if let Some(next) =
            path.pop().or_else(|| super::random_room(rng, &candidates))
        {
            current = next;
        } else {
            break;
        }
    }

    maze
}

use crate::Maze;

use crate::matrix;

/// Initialises a maze using the _Randomised Prim_ algorithm.
///
/// # Arguments
/// *  `maze` - The maze to initialise.
/// *  `rng` - A random number generator.
/// *  `filter` - A predicate filtering rooms to consider.
pub fn initialize<F, R>(mut maze: Maze, rng: &mut R, filter: F) -> Maze
where
    F: Fn(matrix::Pos) -> bool,
    R: super::Randomizer + Sized,
{
    // Create the visited matrix by applying the filter to each room; if no
    // rooms remain we terminate early
    let mut visited = matrix::Matrix::<bool>::new(maze.width(), maze.height());
    let count = visited.positions().fold(0, |mut count, pos| {
        if filter(pos) {
            count += 1;
        } else {
            visited[pos] = true;
        }
        count
    });
    if count == 0 {
        return maze;
    }

    loop {
        // Start with all walls in a random room, except for those leading
        // out of the maze
        let mut walls = visited
            // Pick a random room
            .positions()
            .filter(|&pos| filter(pos))
            .nth(rng.range(0, count))
            // Get all walls not leading out of the maze
            .map(|pos| {
                maze.walls(pos)
                    .iter()
                    .filter(|wall| {
                        maze.rooms.is_inside(maze.back((pos, wall)).0)
                    })
                    // Create a wall position
                    .map(|wall| (pos, *wall))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_else(Vec::<_>::new);

        while !walls.is_empty() {
            // Get a random wall
            let index = rng.range(0, walls.len());
            let wall_pos = walls.remove(index);

            // Walk through the wall if we have not visited the room on the
            // other side before
            let (next_pos, _) = maze.back(wall_pos);
            if !visited[next_pos] {
                // Mark the rooms as visited and open the door
                visited[wall_pos.0] = true;
                visited[next_pos] = true;
                maze.open(wall_pos);

                // Add all walls of the next room except those already
                // visited and those outside of the maze
                walls.extend(
                    maze.walls(next_pos)
                        .iter()
                        .map(|w| maze.back((next_pos, w)))
                        .filter(|&(pos, _)| !visited.get(pos).unwrap_or(&true))
                        .map(|wall_pos| maze.back(wall_pos))
                        .filter(|&(pos, _)| visited.is_inside(pos)),
                );
            }
        }

        if visited.positions().all(|pos| visited[pos]) {
            break;
        }
    }

    maze
}

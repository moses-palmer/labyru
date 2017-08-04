use rand;

use Maze;

use matrix;


pub trait RandomizedPrim<R>
where
    R: rand::Rng + Sized,
{
    /// Initialises a wall using the _Randomised Prim_ algorithm.
    ///
    /// See [here](https://en.wikipedia.org/wiki/Maze_generation_algorithm) for
    /// a description of the algorithm.
    ///
    /// # Arguments
    /// *  `maze` - The maze to initialise. This should be a fully closed maze;
    ///    any already open walls will be ignored and kept.
    /// *  `rng` - A random number generator.
    fn randomized_prim(&mut self, rng: &mut R) -> &mut Self;
}


impl<'a, R> RandomizedPrim<R> for Maze + 'a
where
    R: rand::Rng + Sized,
{
    fn randomized_prim(&mut self, rng: &mut R) -> &mut Self {
        // All rooms that have been visited
        let mut visited =
            matrix::Matrix::<bool>::new(self.width(), self.height());

        // The starting position
        let start = (
            rng.gen_range(0, self.width() as isize),
            rng.gen_range(0, self.height() as isize),
        );

        // Start with all walls in the start room, except for those leading out
        // of the maze
        let mut walls = self.walls(start)
            .iter()
            .filter(|wall| self.rooms().is_inside(self.back(start, wall).0))
            .map(|wall| (start, *wall))
            .collect::<Vec<_>>();

        while !walls.is_empty() {
            // Get a random wall
            let index = rng.gen_range(0, walls.len());
            let (pos, wall) = walls.remove(index);

            // Walk through the wall if we have not visited the room on the
            // other side before
            let (next_pos, _) = self.back(pos, wall);
            if !visited[next_pos] {
                // Mark the rooms as visited and open the door
                visited[pos] = true;
                visited[next_pos] = true;
                self.open(pos, wall);

                // Add all walls of the next room except those already visited
                // and those outside of the maze
                walls.extend(
                    self.walls(next_pos)
                        .iter()
                        .map(|w| self.back(next_pos, w))
                        .filter(|&(p, _)| visited.is_inside(p) && !visited[p])
                        .map(|(p, w)| self.back(p, w)),
                );
            }
        }

        self
    }
}


#[cfg(test)]
mod tests {
    use ::*;
    use super::*;


    fn initialize_randomized_prim(maze: &mut Maze) {
        maze.randomized_prim(&mut rand::weak_rng());

        let from = (0, 0);
        let to = ((maze.width() - 1) as isize, (maze.height() - 1) as isize);
        assert!(maze.walk(from, to).is_some());
    }

    maze_test!(initialize_randomized_prim, initialize_randomized_prim_test);
}

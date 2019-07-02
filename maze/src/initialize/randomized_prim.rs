use crate::Maze;

use crate::matrix;

impl Maze {
    /// Initialises a wall using the _Randomised Prim_ algorithm.
    ///
    /// See [here](https://en.wikipedia.org/wiki/Maze_generation_algorithm) for
    /// a description of the algorithm.
    ///
    /// # Arguments
    /// *  `maze` - The maze to initialise. This should be a fully closed maze;
    ///    any already open walls will be ignored and kept.
    /// *  `rng` - A random number generator.
    pub fn randomized_prim<R>(&mut self, rng: &mut R) -> &mut Self
    where
        R: super::Randomizer + Sized,
    {
        self.randomized_prim_filter(rng, |_| true)
    }

    /// Initialises a wall using the _Randomised Prim_ algorithm.
    ///
    /// See [here](https://en.wikipedia.org/wiki/Maze_generation_algorithm) for
    /// a description of the algorithm.
    ///
    /// This method will ignore rooms for which `filter` returns `false`.
    ///
    /// # Arguments
    /// *  `maze` - The maze to initialise. This should be a fully closed maze;
    ///    any already open walls will be ignored and kept.
    /// *  `rng` - A random number generator.
    /// *  `filter` - A predicate filtering rooms to consider.
    pub fn randomized_prim_filter<F, R>(
        &mut self,
        rng: &mut R,
        filter: F,
    ) -> &mut Self
    where
        F: Fn(matrix::Pos) -> bool,
        R: super::Randomizer + Sized,
    {
        // Create the visited matrix by applying the filter to each room; if no
        // rooms remain we terminate early
        let mut visited =
            matrix::Matrix::<bool>::new(self.width(), self.height());
        let count = visited.positions().fold(0, |mut count, pos| {
            if filter(pos) {
                count += 1;
            } else {
                visited[pos] = true;
            }
            count
        });
        if count == 0 {
            return self;
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
                    self.walls(pos)
                        .iter()
                        .filter(|wall| {
                            self.rooms.is_inside(self.back((pos, wall)).0)
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
                let (next_pos, _) = self.back(wall_pos);
                if !visited[next_pos] {
                    // Mark the rooms as visited and open the door
                    visited[wall_pos.0] = true;
                    visited[next_pos] = true;
                    self.open(wall_pos);

                    // Add all walls of the next room except those already
                    // visited and those outside of the maze
                    walls.extend(
                        self.walls(next_pos)
                            .iter()
                            .map(|w| self.back((next_pos, w)))
                            .filter(|&(pos, _)| {
                                !visited.get(pos).unwrap_or(&true)
                            })
                            .map(|wall_pos| self.back(wall_pos))
                            .filter(|&(pos, _)| visited.is_inside(pos)),
                    );
                }
            }

            if visited.positions().all(|pos| visited[pos]) {
                break;
            }
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use maze_test::maze_test;

    use super::*;
    use crate::test_utils::*;
    use crate::*;

    #[maze_test]
    fn initialize_randomized_prim(mut maze: Maze) {
        maze.randomized_prim(&mut rand::weak_rng());

        let from = matrix_pos(0, 0);
        let to = matrix_pos(
            (maze.width() - 1) as isize,
            (maze.height() - 1) as isize,
        );
        assert!(maze.walk(from, to).is_some());
    }

    #[maze_test]
    fn randomized_prim_filter_most(mut maze: Maze) {
        let from = matrix_pos(0, 0);
        let other = matrix_pos(1, 0);
        let to = matrix_pos(
            (maze.width() - 1) as isize,
            (maze.height() - 1) as isize,
        );
        maze.randomized_prim_filter(&mut rand::weak_rng(), |pos| pos != from);

        assert!(maze.walk(from, to).is_none());
        assert!(maze.walk(other, to).is_some());
    }

    #[maze_test]
    fn randomized_prim_filter_all(mut maze: Maze) {
        let from = matrix_pos(0, 0);
        let other = matrix_pos(1, 0);
        let to = matrix_pos(
            (maze.width() - 1) as isize,
            (maze.height() - 1) as isize,
        );
        maze.randomized_prim_filter(&mut rand::weak_rng(), |_| false);

        assert!(maze.walk(from, to).is_none());
        assert!(maze.walk(other, to).is_none());
    }

    #[maze_test]
    fn randomized_prim_filter_picked(mut maze: Maze) {
        for _ in 0..1000 {
            let filter = |matrix::Pos { col, row }| col > row;
            maze.randomized_prim_filter(&mut rand::weak_rng(), &filter);

            for pos in maze.rooms.positions() {
                assert_eq!(filter(pos), maze.rooms[pos].visited,);
            }
        }
    }

    #[maze_test]
    fn randomized_prim_filter_segmented(mut maze: Maze) {
        for _ in 0..1000 {
            let width = maze.width();
            let height = maze.height();
            let filter = |matrix::Pos { col, row }| {
                col as usize != width / 2 && row as usize != height / 2
            };
            maze.randomized_prim_filter(&mut rand::weak_rng(), &filter);

            for pos in maze.rooms.positions() {
                assert_eq!(filter(pos), maze.rooms[pos].visited,);
            }
        }
    }
}

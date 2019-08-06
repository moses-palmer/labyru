use std::iter;
use std::str;
use std::u64;

use serde::{Deserialize, Serialize};

#[cfg(feature = "osrand")]
use rand;

use crate::Maze;

use crate::matrix;

mod randomized_prim;

/// The various supported initialisation method.
#[derive(Copy, Clone, Debug)]
pub enum Method {
    /// Initialises a maze using a branching algorithm.
    ///
    /// This method uses the _Randomised Prim_ algorithm to generate a maze,
    /// which yields mazes with a branching characteristic.
    ///
    /// See [here](https://en.wikipedia.org/wiki/Maze_generation_algorithm) for
    /// a description of the algorithm.
    Branching,
}

impl str::FromStr for Method {
    type Err = String;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        match source {
            "branching" => Ok(Method::Branching),
            e => Err(e.to_owned()),
        }
    }
}

pub trait Randomizer {
    /// Generates a random value in the range `[low, high)`, where `low` and
    /// `high` are the low and high values of `a` and `b`.
    ///
    /// # Arguments
    /// *  `a` - A number.
    /// *  `b` - A number.
    fn range(&mut self, a: usize, b: usize) -> usize;

    /// Generates a random value in the range `[0, 1)`.
    fn random(&mut self) -> f64;
}

#[cfg(feature = "osrand")]
impl<T> Randomizer for T
where
    T: rand::Rng,
{
    fn range(&mut self, a: usize, b: usize) -> usize {
        if a < b {
            self.gen_range(a, b)
        } else {
            self.gen_range(b, a)
        }
    }

    fn random(&mut self) -> f64 {
        self.next_f64()
    }
}

/// A linear feedback shift register.
#[derive(Deserialize, Serialize)]
pub struct LFSR(u64);

impl LFSR {
    /// Creates a new linear shift register.
    ///
    /// # Arguments
    /// *  `seed` - The seed. This value will not be yielded.
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }

    /// Advances this shift register by one `u64` and returns the bit mask.
    pub fn advance(&mut self) -> u64 {
        self.nth(63).unwrap();

        self.0
    }
}

impl iter::Iterator for LFSR {
    type Item = bool;

    /// Returns the next bit.
    fn next(&mut self) -> Option<Self::Item> {
        let bit = (self.0 ^ (self.0 >> 2) ^ (self.0 >> 3) ^ (self.0 >> 5)) & 1;

        self.0 = (self.0 >> 1) | (bit << 63);

        Some(bit != 0)
    }
}

impl Randomizer for LFSR {
    fn range(&mut self, a: usize, b: usize) -> usize {
        let val = self.advance() as usize;
        let (low, high) = if a < b { (a, b) } else { (b, a) };
        if low == high {
            low
        } else {
            low + val % (high - low)
        }
    }

    fn random(&mut self) -> f64 {
        self.advance() as f64 / u64::MAX as f64
    }
}

impl Maze {
    /// Initialises a maze using the selected algorithm.
    ///
    /// See [here](https://en.wikipedia.org/wiki/Maze_generation_algorithm) for
    /// a description of the algorithms.
    ///
    /// The maze  should be fully closed; any already open walls will be
    /// ignored and kept.
    ///
    /// # Arguments
    /// *  `method` - The initialisation method to use.
    /// *  `rng` - A random number generator.
    pub fn initialize<R>(self, method: Method, rng: &mut R) -> Self
    where
        R: Randomizer + Sized,
    {
        self.initialize_filter(method, rng, |_| true)
    }

    /// Initialises a maze using the selected algorithm.
    ///
    /// See [here](https://en.wikipedia.org/wiki/Maze_generation_algorithm) for
    /// a description of the algorithms.
    ///
    /// The maze  should be fully closed; any already open walls will be
    /// ignored and kept.
    ///
    /// # Arguments
    /// *  `method` - The initialisation method to use.
    /// *  `rng` - A random number generator.
    /// *  `filter` - A filter function used to ignore rooms.
    pub fn initialize_filter<R, F>(
        self,
        method: Method,
        rng: &mut R,
        filter: F,
    ) -> Self
    where
        F: Fn(matrix::Pos) -> bool,
        R: Randomizer + Sized,
    {
        match method {
            Method::Branching => randomized_prim::initialize(self, rng, filter),
        }
    }
}

#[cfg(test)]
mod tests {
    use maze_test::maze_test;

    use super::*;
    use crate::test_utils::*;
    use crate::*;

    /// The various initialisation methods tested.
    const INITIALIZERS: &[Method] = &[Method::Branching];

    /// Tests that range works as advertised.
    #[test]
    fn lfsr_range() {
        let mut lfsr = LFSR::new(12345);
        for a in 0..100 {
            for b in a..a + 100 {
                for _ in 0..100 {
                    let v = lfsr.range(a, b);
                    if !(a <= v && v < b) {
                        println!("!({} <= {} < {})", a, v, b);
                    }
                    if b > a {
                        assert!(a <= v && v < b);
                    } else {
                        assert!(a == v && v == b);
                    }
                }
            }
        }
    }

    /// Tests that random gives a rectangular distribution.
    #[test]
    fn lfsr_random() {
        let mut lfsr = LFSR::new(12345);

        let buckets = 100;
        let iterations = 100 * 100 * buckets;
        let hist = (0..iterations).fold(vec![0; buckets], |mut hist, _| {
            hist[(buckets as f64 * lfsr.random()) as usize] += 1;
            hist
        });

        let mid = iterations / buckets;
        let h = 400;
        for v in hist {
            assert!(mid - h < v && v < mid + h);
        }
    }

    #[maze_test]
    fn initialize(maze: Maze) {
        for method in INITIALIZERS {
            let maze = maze.clone().initialize(*method, &mut rand::weak_rng());

            let from = matrix_pos(0, 0);
            let to = matrix_pos(
                (maze.width() - 1) as isize,
                (maze.height() - 1) as isize,
            );
            assert!(maze.walk(from, to).is_some());
        }
    }

    #[maze_test]
    fn initialize_filter_most(maze: Maze) {
        for method in INITIALIZERS {
            let from = matrix_pos(0, 0);
            let other = matrix_pos(1, 0);
            let to = matrix_pos(
                (maze.width() - 1) as isize,
                (maze.height() - 1) as isize,
            );
            let maze = maze.clone().initialize_filter(
                *method,
                &mut rand::weak_rng(),
                |pos| pos != from,
            );

            assert!(maze.walk(from, to).is_none());
            assert!(maze.walk(other, to).is_some());
        }
    }

    #[maze_test]
    fn initialize_filter_all(maze: Maze) {
        for method in INITIALIZERS {
            let from = matrix_pos(0, 0);
            let other = matrix_pos(1, 0);
            let to = matrix_pos(
                (maze.width() - 1) as isize,
                (maze.height() - 1) as isize,
            );
            let maze = maze.clone().initialize_filter(
                *method,
                &mut rand::weak_rng(),
                |_| false,
            );

            assert!(maze.walk(from, to).is_none());
            assert!(maze.walk(other, to).is_none());
        }
    }

    #[maze_test]
    fn initialize_filter_picked(maze: Maze) {
        for method in INITIALIZERS {
            for _ in 0..1000 {
                let filter = |matrix::Pos { col, row }| col > row;
                let maze = maze.clone().initialize_filter(
                    *method,
                    &mut rand::weak_rng(),
                    &filter,
                );

                for pos in maze.rooms.positions() {
                    assert_eq!(filter(pos), maze.rooms[pos].visited);
                }
            }
        }
    }

    #[maze_test]
    fn initialize_filter_segmented(maze: Maze) {
        for method in INITIALIZERS {
            for _ in 0..1000 {
                let width = maze.width();
                let height = maze.height();
                let filter = |matrix::Pos { col, row }| {
                    col as usize != width / 2 && row as usize != height / 2
                };
                let maze = maze.clone().initialize_filter(
                    *method,
                    &mut rand::weak_rng(),
                    &filter,
                );

                for pos in maze.rooms.positions() {
                    assert_eq!(filter(pos), maze.rooms[pos].visited);
                }
            }
        }
    }
}

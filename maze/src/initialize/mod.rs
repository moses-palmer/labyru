//! # Initialisation methods
//!
//! This module contains implementations of initialisation methods. These are
//! used to open walls in a fully closed maze to make it navigable.

use std::iter;
use std::str;
use std::u64;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::Maze;

use crate::matrix;

mod braid;
mod branching;
mod clear;
mod winding;

/// The various supported initialisation method.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Method {
    /// Initialises a maze with no dead ends.
    ///
    /// A dead end is a room with only one open wall.
    ///
    /// This method starts with a fully cleared area, and adds walls until no
    /// longer possible without creating dead ends. A maze initialised with
    /// this method will contain loops.
    Braid,

    /// Initialises a maze by opening all walls inside the area.
    Clear,

    /// Initialises a maze using a branching algorithm.
    ///
    /// This method uses the _Randomised Prim_ algorithm to generate a maze,
    /// which yields mazes with a branching characteristic. A maze initialised
    /// with this method will not contain loops.
    ///
    /// See [Wikipedia] for a description of the algorithm.
    ///
    /// [Wikipedia]: https://en.wikipedia.org/wiki/Maze_generation_algorithm#Randomized_Prim's_algorithm
    Branching,

    /// Initialises a maze using a winding algorithm.
    ///
    /// This method uses a simple _Depth First_ algorithm to generate a maze,
    /// which yields mazes with long winding corridors. A maze initialised with
    /// this method will not contain loops.
    ///
    /// See [Wikipedia] for a description of the algorithm.
    ///
    /// [Wikipedia]: https://en.wikipedia.org/wiki/Maze_generation_algorithm#Depth-first_search
    Winding,
}

impl Default for Method {
    /// The default initialisation method is [`Branching`](Method::Branchin).
    fn default() -> Self {
        Method::Branching
    }
}

impl std::fmt::Display for Method {
    /// The opposite of [std::str::FromStr].
    ///
    /// # Examples
    ///
    /// ```
    /// # use maze::initialize::*;
    ///
    /// assert_eq!(
    ///     Method::Braid.to_string().parse::<Method>(),
    ///     Ok(Method::Braid),
    /// );
    /// assert_eq!(
    ///     Method::Branching.to_string().parse::<Method>(),
    ///     Ok(Method::Branching),
    /// );
    /// assert_eq!(
    ///     Method::Clear.to_string().parse::<Method>(),
    ///     Ok(Method::Clear),
    /// );
    /// assert_eq!(
    ///     Method::Winding.to_string().parse::<Method>(),
    ///     Ok(Method::Winding),
    /// );
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Method::*;
        match self {
            Braid => write!(f, "braid"),
            Clear => write!(f, "clear"),
            Branching => write!(f, "branching"),
            Winding => write!(f, "winding"),
        }
    }
}

impl str::FromStr for Method {
    type Err = String;

    /// Converts a string to an initialiser.
    ///
    /// The source strings are the lower case names of the initialisation
    /// methods.
    ///
    /// # Examples
    ///
    /// ```
    /// # use maze::initialize::*;
    ///
    /// assert_eq!(
    ///     "braid".parse::<Method>(),
    ///     Ok(Method::Braid),
    /// );
    /// assert_eq!(
    ///     "branching".parse::<Method>(),
    ///     Ok(Method::Branching),
    /// );
    /// assert_eq!(
    ///     "clear".parse::<Method>(),
    ///     Ok(Method::Clear),
    /// );
    /// assert_eq!(
    ///     "winding".parse::<Method>(),
    ///     Ok(Method::Winding),
    /// );
    /// ```
    fn from_str(source: &str) -> Result<Self, Self::Err> {
        match source {
            "braid" => Ok(Method::Braid),
            "clear" => Ok(Method::Clear),
            "branching" => Ok(Method::Branching),
            "winding" => Ok(Method::Winding),
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

#[cfg(feature = "rand")]
impl<T> Randomizer for T
where
    T: rand::Rng,
{
    fn range(&mut self, a: usize, b: usize) -> usize {
        if a < b {
            self.gen_range(a..b)
        } else {
            self.gen_range(b..a)
        }
    }

    fn random(&mut self) -> f64 {
        self.gen()
    }
}

/// A linear feedback shift register.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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

impl<T> From<T> for LFSR
where
    T: Into<u64>,
{
    fn from(source: T) -> Self {
        Self(source.into())
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

impl<T> Maze<T>
where
    T: Clone,
{
    /// Initialises a maze using the selected algorithm.
    ///
    /// See [here](https://en.wikipedia.org/wiki/Maze_generation_algorithm) for
    /// a description of the algorithms.
    ///
    /// The maze  should be fully closed; any already open walls will be
    /// ignored and kept.
    ///
    /// This method guarantees that the resulting maze is predictable if the
    /// _RNG_ is predictable.
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
    /// This method guarantees that the resulting maze is predictable if the
    /// _RNG_ is predictable.
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
        match matrix::filter(self.width(), self.height(), filter) {
            (count, filter) if count > 0 => match method {
                Method::Braid => braid::initialize(self, rng, filter),
                Method::Clear => clear::initialize(self, rng, filter),
                Method::Branching => branching::initialize(self, rng, filter),
                Method::Winding => winding::initialize(self, rng, filter),
            },
            _ => self,
        }
    }
}

/// Returns a random unvisited room.
///
/// # Arguments
/// *  `rng` - A random number generator.
/// *  `filter_matrix` - A matrix containing the rooms to consider.
fn random_room(
    rng: &mut dyn Randomizer,
    filter_matrix: &matrix::Matrix<bool>,
) -> Option<matrix::Pos> {
    let count = filter_matrix
        .positions()
        .filter(|&pos| filter_matrix[pos])
        .count();
    if count > 0 {
        filter_matrix
            .positions()
            .filter(|&pos| filter_matrix[pos])
            .nth(rng.range(0, count))
    } else {
        None
    }
}

/// Ensures all rooms are connected
///
/// This function will find all closed areas and ensure they have one exit to
/// each neighbouring area.
///
/// # Arguments
/// *  `maze` - The maze to modify.
/// *  `filter` - A filter for rooms to consider.
pub fn connect_all<F, R, T>(maze: &mut Maze<T>, rng: &mut R, filter: F)
where
    F: Fn(matrix::Pos) -> bool,
    R: Randomizer + Sized,
    T: Clone,
{
    // First find all non-connected areas by visiting all rooms and filling for
    // each filtered, non-filled room and then incrementing the area index
    let mut areas = matrix::Matrix::new(maze.width(), maze.height());
    let mut index = 0;
    for pos in maze.positions() {
        // Ignore filtered and already visited rooms
        if !filter(pos) || areas[pos] > 0 {
            continue;
        } else {
            index += 1;
            areas.fill(pos, index, |pos| {
                maze.neighbors(pos).filter(|&pos| filter(pos))
            });
        }
    }

    // Then find all edges between separate areas and open a random wall
    for (_, edge) in areas
        .edges(|pos| maze.adjacent(pos))
        .iter()
        .filter(|&((source, _), _)| *source > 0)
    {
        let wall_positions = edge
            .iter()
            .flat_map(|&(pos1, pos2)| maze.connecting_wall(pos1, pos2))
            .collect::<Vec<_>>();
        maze.open(wall_positions[rng.range(0, wall_positions.len())])
    }
}

#[cfg(test)]
mod tests {
    use maze_test::maze_test;

    use super::*;
    use crate::test_utils::*;

    /// The various initialisation methods tested.
    const INITIALIZERS: &[Method] =
        &[Method::Braid, Method::Branching, Method::Winding];

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

    #[test]
    fn random_room_none() {
        let width = 5;
        let height = 5;
        let mut rng = LFSR::new(12345);
        let (count, filter_matrix) = matrix::filter(width, height, |_| false);

        assert_eq!(0, count);

        let iterations = width * height * 100;
        for _ in 0..iterations {
            assert!(random_room(&mut rng, &filter_matrix).is_none());
        }
    }

    #[test]
    fn random_room_some() {
        let width = 5;
        let height = 5;
        let mut rng = LFSR::new(12345);
        let (count, filter_matrix) =
            matrix::filter(width, height, |pos| pos.col as usize == width - 1);

        assert_eq!(height, count);

        let buckets = height;
        let iterations = 100 * 100 * buckets;
        let hist = (0..iterations).fold(vec![0; buckets], |mut hist, _| {
            hist[random_room(&mut rng, &filter_matrix).unwrap().row
                as usize] += 1;
            hist
        });

        let mid = iterations / buckets;
        let h = 400;
        for v in hist {
            assert!(mid - h < v && v < mid + h);
        }
    }

    #[maze_test]
    fn initialize(maze: TestMaze) {
        for method in INITIALIZERS {
            let maze =
                maze.clone().initialize(*method, &mut rand::thread_rng());

            let from = matrix_pos(0, 0);
            let to = matrix_pos(
                (maze.width() - 1) as isize,
                (maze.height() - 1) as isize,
            );
            assert!(maze.walk(from, to).is_some());
        }
    }

    #[maze_test]
    fn initialize_lfsr_stable(maze: TestMaze) {
        for method in INITIALIZERS {
            let seed = 12345;
            let mut rng1 = LFSR::new(seed);
            let mut rng2 = LFSR::new(seed);
            maze.clone().initialize(*method, &mut rng1);
            maze.clone().initialize(*method, &mut rng2);

            assert_eq!(rng1, rng2, "for method {:?}", method);
        }
    }

    #[maze_test]
    fn initialize_filter_most(maze: TestMaze) {
        for method in INITIALIZERS {
            let from = matrix_pos(0, 0);
            let other = matrix_pos(1, 0);
            let to = matrix_pos(
                (maze.width() - 1) as isize,
                (maze.height() - 1) as isize,
            );
            let maze = maze.clone().initialize_filter(
                *method,
                &mut rand::thread_rng(),
                |pos| pos != from,
            );

            assert!(maze.walk(from, to).is_none());
            assert!(maze.walk(other, to).is_some());
        }
    }

    #[maze_test]
    fn initialize_filter_all(maze: TestMaze) {
        for method in INITIALIZERS {
            let from = matrix_pos(0, 0);
            let other = matrix_pos(1, 0);
            let to = matrix_pos(
                (maze.width() - 1) as isize,
                (maze.height() - 1) as isize,
            );
            let maze = maze.clone().initialize_filter(
                *method,
                &mut rand::thread_rng(),
                |_| false,
            );

            assert!(maze.walk(from, to).is_none());
            assert!(maze.walk(other, to).is_none());
        }
    }

    #[maze_test]
    fn initialize_filter_picked(maze: TestMaze) {
        for method in INITIALIZERS {
            for _ in 0..1000 {
                let filter = |matrix::Pos { col, row }| col > row;
                let maze = maze.clone().initialize_filter(
                    *method,
                    &mut rand::thread_rng(),
                    filter,
                );

                for pos in maze.positions() {
                    assert_eq!(filter(pos), maze[pos].visited);
                }
            }
        }
    }

    #[maze_test]
    fn initialize_filter_segmented(maze: TestMaze) {
        for method in INITIALIZERS {
            for _ in 0..1000 {
                let width = maze.width();
                let height = maze.height();
                let filter = |matrix::Pos { col, row }| {
                    col as usize != width / 2 && row as usize != height / 2
                };
                let maze = maze.clone().initialize_filter(
                    *method,
                    &mut rand::thread_rng(),
                    filter,
                );

                for pos in maze.positions() {
                    assert_eq!(filter(pos), maze[pos].visited);
                }
            }
        }
    }
}

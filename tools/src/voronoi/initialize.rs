use std::iter;

use maze::initialize;
use maze::matrix;
use maze::physical;

/// A container struct for multiple initialisation methods.
#[derive(Clone, Debug)]
pub struct Methods<R>
where
    R: initialize::Randomizer + Sized,
{
    methods: Vec<initialize::Method>,

    _marker: ::std::marker::PhantomData<R>,
}

/// A description of an initialised maze and its areas.
pub struct InitializedMaze<T>
where
    T: Clone,
{
    /// The initialised maze.
    pub maze: maze::Maze<T>,

    /// A mapping from room position to the index of its initialiser in the
    /// initialisation vector.
    pub areas: matrix::Matrix<usize>,
}

impl<T> InitializedMaze<T>
where
    T: Clone,
{
    /// Maps each room of the maze, yielding a maze with the same layout but
    /// with transformed data.
    ///
    /// This method allows for incorporating are information into the new maze.
    ///
    /// # Arguments
    /// *  `data` - A function providing data for the new maze.
    pub fn map<F, U>(&self, mut data: F) -> maze::Maze<U>
    where
        F: FnMut(matrix::Pos, T, usize) -> U,
        U: Clone,
    {
        self.maze
            .map(|pos, value| data(pos, value, self.areas[pos]))
    }
}

impl<T> From<InitializedMaze<T>> for maze::Maze<T>
where
    T: Clone,
{
    fn from(source: InitializedMaze<T>) -> Self {
        source.maze
    }
}

impl<R> Methods<R>
where
    R: initialize::Randomizer + Sized,
{
    /// Creates an initialiser for a list of initialisation methods.
    ///
    /// # Arguments
    /// *  `methods` - The initialisation methods to use.
    pub fn new(methods: Vec<initialize::Method>) -> Self {
        Self {
            methods,
            _marker: ::std::marker::PhantomData,
        }
    }

    /// The initialisation methods.
    pub fn methods(&self) -> &Vec<initialize::Method> {
        &self.methods
    }

    /// Initialises a maze by applying all methods defined for this collection.
    ///
    /// This method generates a Voronoi diagram for all methods with centres and
    /// weights from `points`, and uses that and the `filter` argument to limit
    /// each initialisation method.
    ///
    /// The matrix returned is the Voronoi diagram used, where values are
    /// indices in the `methods` vector.
    ///
    /// # Arguments
    /// *  `maze` - The maze to initialise.
    /// *  `rng` - A random number generator.
    /// *  `filter` - An additional filter applied to all methods.
    /// *  `points` - The points and weights to generate a Voronoi diagram.
    pub fn initialize<F, T, P>(
        self,
        maze: maze::Maze<T>,
        rng: &mut R,
        filter: F,
        points: P,
    ) -> InitializedMaze<T>
    where
        F: Fn(matrix::Pos) -> bool,
        T: Clone,
        P: Iterator<Item = super::Point<usize>>,
    {
        // Generate the areas
        let areas =
            super::matrix(&maze, points.take(self.methods.len()).collect());

        // Use a different initialisation method for each segment
        let mut maze = self.methods.into_iter().enumerate().fold(
            maze,
            |maze, (i, method)| {
                maze.initialize_filter(method, rng, |pos| {
                    filter(pos) && areas[pos] == i
                })
            },
        );

        // Make sure all segments are connected
        initialize::connect_all(&mut maze, rng, filter);

        InitializedMaze { maze, areas }
    }

    /// Generates an infinite enumeration of random points and weights.
    ///
    /// The value of the points yielded is their index.
    ///
    /// # Arguments
    /// *  `viewbox` - The viewbox to which to constrain the points.
    /// *  `rng``- A random number generator.
    pub fn random_points(
        viewbox: physical::ViewBox,
        rng: &mut R,
    ) -> impl Iterator<Item = super::Point<usize>> + '_ {
        iter::repeat_with(move || {
            (
                physical::Pos {
                    x: viewbox.corner.x + rng.random() as f32 * viewbox.width,
                    y: viewbox.corner.y + rng.random() as f32 * viewbox.height,
                },
                (rng.random() as f32) + 0.5,
            )
        })
        .enumerate()
    }
}

impl<R> Default for Methods<R>
where
    R: initialize::Randomizer + Sized,
{
    fn default() -> Self {
        Self {
            methods: vec![initialize::Method::default()],
            _marker: ::std::marker::PhantomData,
        }
    }
}

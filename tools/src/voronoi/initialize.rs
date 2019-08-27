use maze;
use maze::initialize;
use maze::matrix;
use maze::physical;

/// A container struct for multiple initialisation methods.
pub struct Methods<R>
where
    R: initialize::Randomizer + Sized,
{
    methods: Vec<initialize::Method>,

    _marker: ::std::marker::PhantomData<R>,
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

    /// Initialises a maze by applying all methods defined for this collection.
    ///
    /// This method generates a Voronoi diagram for all methods with random
    /// centres and weights, and uses that and the `filter` argument to limit
    /// each initialisation method.
    ///
    /// The matrix returned is the Voronoi diagram used, where values are
    /// indice in the `methods` vector.
    ///
    /// # Arguments
    /// *  `maze` - The maze to initialise.
    /// *  `rng` - A random number generator.
    /// *  `filter` - An additional filter applied to all methods.
    pub fn initialize<F>(
        self,
        maze: maze::Maze,
        rng: &mut R,
        filter: F,
    ) -> (matrix::Matrix<usize>, maze::Maze)
    where
        F: Fn(matrix::Pos) -> bool,
    {
        // Generate the segments and find all edges
        let matrix = self.matrix(&maze, rng);
        let edges =
            matrix.edges(|pos| maze.wall_positions(pos).map(|(pos, _)| pos));

        // Use a different initialisation method for each segment
        let mut maze = self.methods.into_iter().enumerate().fold(
            maze,
            |maze, (i, method)| {
                maze.initialize_filter(method, rng, |pos| {
                    filter(pos) && matrix[pos] == i
                })
            },
        );

        // Make sure all segments are connected
        for edge in edges.values() {
            let wall_positions = edge
                .iter()
                .flat_map(|&(pos1, pos2)| maze.connecting_wall(pos1, pos2))
                .collect::<Vec<_>>();
            maze.open(wall_positions[rng.range(0, wall_positions.len())])
        }

        (matrix, maze)
    }

    /// Generates a Voronoi diagram where values are indices into the methods
    /// vector.
    ///
    /// # Arguments
    /// *  `maze` - The source maze.
    fn matrix(&self, maze: &maze::Maze, rng: &mut R) -> matrix::Matrix<usize> {
        let (left, top, width, height) = maze.viewbox();
        super::matrix(
            maze,
            (0..self.methods.len())
                .map(|i| {
                    (
                        physical::Pos {
                            x: left + rng.random() as f32 * width,
                            y: top + rng.random() as f32 * height,
                        },
                        (rng.random() as f32) + 0.5,
                        i,
                    )
                })
                .collect(),
        )
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

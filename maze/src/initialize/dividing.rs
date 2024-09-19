use crate::Maze;

use crate::matrix;
use crate::physical;

/// Initialises a maze using the _Randomised Prim_ algorithm.
///
/// # Arguments
/// *  `maze` - The maze to initialise.
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
    // We need to work with a clear maze
    for pos in maze.positions().filter(|&pos| candidates[pos]) {
        for wall in maze.walls(pos) {
            let (pos, wall) = maze.back((pos, wall));
            if *candidates.get(pos).unwrap_or(&false) {
                maze.open((pos, wall));
            }
        }
    }

    // Calculate the full view box of our candidate area
    let viewbox = maze
        .positions()
        .flat_map(|pos| maze.wall_positions(pos))
        .map(|wall_pos| maze.corners(wall_pos).0)
        .collect::<physical::ViewBox>();

    // Stop recursing when any split has a side less than twice the distance
    // between two rooms
    let threshold = 2.0
        * (maze.center((0isize, 0isize).into())
            - maze.center((1isize, 1isize).into()))
        .value()
        .sqrt();

    // Recursively split and rebuild
    Split::from_viewbox(viewbox, rng).apply(
        &mut maze,
        rng,
        &candidates,
        threshold,
    );

    maze
}

enum Split {
    Horizontal(physical::ViewBox, f32),
    Vertical(physical::ViewBox, f32),
}

impl Split {
    pub fn from_viewbox<R>(viewbox: physical::ViewBox, rng: &mut R) -> Self
    where
        R: super::Randomizer + Sized,
    {
        let cut = 0.8 * rng.random() as f32 + 0.2;
        if viewbox.width > viewbox.height {
            Self::Vertical(viewbox, viewbox.corner.x + cut * viewbox.width)
        } else {
            Self::Horizontal(viewbox, viewbox.corner.y + cut * viewbox.height)
        }
    }

    pub fn apply<R, T>(
        self,
        maze: &mut Maze<T>,
        rng: &mut R,
        candidates: &matrix::Matrix<bool>,
        threshold: f32,
    ) where
        R: super::Randomizer + Sized,
        T: Clone,
    {
        use Split::*;

        // Make a random cut
        let ranges = match self {
            Horizontal(viewbox, at) => {
                let (a, b) = (
                    maze.room_at((viewbox.corner.x, at).into()),
                    maze.room_at((viewbox.corner.x + viewbox.width, at).into()),
                );
                ((a.col - 1..a.col + 1), (a.row - 1..b.row + 1))
            }
            Vertical(viewbox, at) => {
                let (a, b) = (
                    maze.room_at((at, viewbox.corner.y).into()),
                    maze.room_at(
                        (at, viewbox.corner.y + viewbox.height).into(),
                    ),
                );
                ((a.col - 1..a.col + 1), (a.row - 1..b.row + 1))
            }
        };

        // Close walls along the cut
        for pos in ranges
            .0
            .flat_map(|x| ranges.1.clone().map(move |y| (x, y).into()))
            .filter(|&pos| *candidates.get(pos).unwrap_or(&false))
        {
            for a in maze.wall_positions(pos) {
                let b = maze.back(a);
                if *candidates.get(b.0).unwrap_or(&false)
                    && (self.contains(maze.center(a.0))
                        != self.contains(maze.center(b.0)))
                {
                    maze.close(a);
                }
            }
        }

        // Recurse
        let splits = self.split(rng);
        for split in [splits.0, splits.1] {
            let viewbox = split.viewbox();
            if [viewbox.width, viewbox.height]
                .iter()
                .all(|&x| x > threshold * (1.0 + rng.random() as f32))
            {
                split.apply(maze, rng, candidates, threshold);
            }
        }
    }

    fn split<R>(self, rng: &mut R) -> (Self, Self)
    where
        R: super::Randomizer + Sized,
    {
        use Split::*;
        let viewboxes = match self {
            Horizontal(viewbox, at) => viewbox.split_horizontal(at),
            Vertical(viewbox, at) => viewbox.split_vertical(at),
        };
        (
            Self::from_viewbox(viewboxes.0, rng),
            Self::from_viewbox(viewboxes.1, rng),
        )
    }

    fn viewbox(&self) -> &physical::ViewBox {
        use Split::*;
        match self {
            Horizontal(viewbox, _) | Vertical(viewbox, _) => viewbox,
        }
    }

    fn contains(&self, pos: physical::Pos) -> bool {
        use Split::*;
        match self {
            Horizontal(_, at) => pos.y < *at,
            Vertical(_, at) => pos.x < *at,
        }
    }
}

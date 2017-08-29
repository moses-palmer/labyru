use super::*;


/// Creates a test function that runs the tests for all known types of
/// mazes.
#[macro_export]
macro_rules! maze_test {
    ($test_function:ident, $name:ident) => {
        #[test]
        fn $name() {
            let width = 10;
            let height = 5;

            $test_function(&mut shape::hex::Maze::new(width, height));
            $test_function(&mut shape::quad::Maze::new(width, height));
            $test_function(&mut shape::tri::Maze::new(width, height));
        }
    }
}


/// Determines whether two physical locations are close enough to be
/// considered equal.
///
/// # Arguments
/// * `expected` - The expected location.
/// * `actual` - Another location.
pub fn is_close(expected: physical::Pos, actual: physical::Pos) -> bool {
    let d = (expected.0 - actual.0, expected.1 - actual.1);
    (d.0 * d.0 + d.1 * d.1).sqrt() < 0.00001
}


/// A navigator through a maze.
///
/// This struct provides utility methods to open and close doors based on
/// directions.
pub struct Navigator<'a> {
    maze: &'a mut Maze,
    pos: matrix::Pos,
}

impl<'a> Navigator<'a> {
    /// Creates a new navigator for a specific maze.
    ///
    /// # Arguments
    /// *  `maze` - The maze to modify.
    pub fn new(maze: &'a mut Maze) -> Navigator<'a> {
        Navigator {
            maze: maze,
            pos: (0, 0),
        }
    }

    /// Moves the navigator to a specific room.
    ///
    /// # Arguments
    /// *  `pos` - The new position.
    pub fn from<'b>(&'b mut self, pos: matrix::Pos) -> &'b mut Self {
        self.pos = pos;
        self
    }

    /// Opens or closes a wall leading _up_.
    ///
    /// The current room position is also updated.
    ///
    /// # Arguments
    /// *  `open` - Whether to open the wall.
    ///
    /// # Panics
    /// This method panics if there is no wall leading up from the current
    /// room.
    pub fn up<'b>(&'b mut self, open: bool) -> &'b mut Self {
        self.navigate(|wall| wall.dir == (0, -1), open)
    }

    /// Opens or closes a wall leading _down_.
    ///
    /// The current room position is also updated.
    ///
    /// # Arguments
    /// *  `open` - Whether to open the wall.
    ///
    /// # Panics
    /// This method panics if there is no wall leading down from the current
    /// room.
    pub fn down<'b>(&'b mut self, open: bool) -> &'b mut Self {
        self.navigate(|wall| wall.dir == (0, 1), open)
    }

    /// Opens or closes a wall leading _left_.
    ///
    /// The current room position is also updated.
    ///
    /// # Arguments
    /// *  `open` - Whether to open the wall.
    ///
    /// # Panics
    /// This method panics if there is no wall leading left from the current
    /// room.
    pub fn left<'b>(&'b mut self, open: bool) -> &'b mut Self {
        self.navigate(|wall| wall.dir == (-1, 0), open)
    }

    /// Opens or closes a wall leading _right_.
    ///
    /// The current room position is also updated.
    ///
    /// # Arguments
    /// *  `open` - Whether to open the wall.
    ///
    /// # Panics
    /// This method panics if there is no wall leading right from the
    /// current room.
    pub fn right<'b>(&'b mut self, open: bool) -> &'b mut Self {
        self.navigate(|wall| wall.dir == (1, 0), open)
    }

    /// Opens or closes a wall.
    ///
    /// The current room position is also updated.
    ///
    /// # Arguments
    /// *  `open` - Whether to open the wall.
    ///
    /// The wall selected is the first one for which `predicate` returns
    /// `true`.
    ///
    /// # Panics
    /// This method panics if there is no wall for which the predicate
    /// returns `true`.
    fn navigate<'b, P>(&'b mut self, predicate: P, open: bool) -> &'b mut Self
    where
        for<'r> P: FnMut(&'r &&wall::Wall) -> bool,
    {
        let wall = self.maze
            .walls(self.pos)
            .iter()
            .filter(predicate)
            .next()
            .unwrap();
        self.maze.set_open((self.pos, wall), open);
        self.pos = (self.pos.0 + wall.dir.0, self.pos.1 + wall.dir.1);
        self
    }
}

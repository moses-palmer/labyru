use super::*;

pub type TestMaze = Maze<()>;

/// Determines whether two physical locations are close enough to be
/// considered equal.
///
/// # Arguments
/// *  `expected` - The expected location.
/// *  `actual` - Another location.
pub fn is_close(expected: physical::Pos, actual: physical::Pos) -> bool {
    let d = (expected.x - actual.x, expected.y - actual.y);
    (d.0 * d.0 + d.1 * d.1).sqrt() < 0.00001
}

/// Determines whether two floating point values are close enough to be
/// considered equal.
///
/// This function lowers the resolution to `std::f32::EPSILON * 4.0`.
///
/// # Arguments
/// *  `a` - One value.
/// *  `b` - Another value.
pub fn nearly_equal(a: f32, b: f32) -> bool {
    return a == b || (a - b).abs() < std::f32::EPSILON * 4.0;
}

/// A simple helper to create a matrix position.
///
/// # Arguments
/// *  `col` - The column.
/// *  `row` - The row.
pub fn matrix_pos(col: isize, row: isize) -> matrix::Pos {
    matrix::Pos { col, row }
}

/// A navigator through a maze.
///
/// This struct provides utility methods to open and close doors based on
/// directions.
pub struct Navigator<'a> {
    maze: &'a mut TestMaze,
    pos: Option<matrix::Pos>,
    log: Vec<matrix::Pos>,
}

impl<'a> Navigator<'a> {
    /// Creates a new navigator for a specific maze.
    ///
    /// # Arguments
    /// *  `maze` - The maze to modify.
    pub fn new(maze: &'a mut TestMaze) -> Navigator<'a> {
        Navigator {
            maze,
            pos: None,
            log: Vec::new(),
        }
    }

    /// Moves the navigator to a specific room.
    ///
    /// # Arguments
    /// *  `pos` - The new position.
    pub fn from(mut self, pos: matrix::Pos) -> Self {
        self.pos = Some(pos);
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
    pub fn up(self, open: bool) -> Self {
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
    pub fn down(self, open: bool) -> Self {
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
    pub fn left(self, open: bool) -> Self {
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
    pub fn right(self, open: bool) -> Self {
        self.navigate(|wall| wall.dir == (1, 0), open)
    }

    /// Stops and freezes this navigator.
    pub fn stop(mut self) -> Vec<matrix::Pos> {
        self.log.push(self.pos.unwrap());
        self.log
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
    pub fn navigate<P>(mut self, mut predicate: P, open: bool) -> Self
    where
        for<'r> P: FnMut(&'r &&wall::Wall) -> bool,
    {
        if self.pos.is_none() {
            self.pos = self
                .maze
                .positions()
                .filter(|&pos| {
                    self.maze.walls(pos).iter().any(|wall| predicate(&wall))
                })
                .next();
        }
        let pos = self.pos.unwrap();
        self.log.push(pos);

        let wall = self
            .maze
            .walls(pos)
            .iter()
            .filter(predicate)
            .filter(|wall| {
                self.maze.is_inside(matrix_pos(
                    pos.col + wall.dir.0,
                    pos.row + wall.dir.1,
                ))
            })
            .next()
            .unwrap();
        self.maze.set_open((pos, wall), open);
        self.pos = Some(matrix_pos(pos.col + wall.dir.0, pos.row + wall.dir.1));
        self
    }
}

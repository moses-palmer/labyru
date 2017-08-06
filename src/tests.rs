use std::collections::HashSet;

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

            $test_function(&mut shape::quad::Maze::new(width, height));
        }
    }
}


fn is_inside_correct(maze: &mut Maze) {
    assert!(maze.rooms().is_inside((0, 0)));
    assert!(maze.rooms().is_inside((
        maze.width() as isize - 1,
        maze.height() as isize - 1,
    )));
    assert!(!maze.rooms().is_inside((-1, -1)));
    assert!(!maze.rooms().is_inside(
        (maze.width() as isize, maze.height() as isize),
    ));
}

maze_test!(is_inside_correct, is_inside_correct_test);


fn can_open(maze: &mut Maze) {
    let pos = (0, 0);
    let next = (0, 1);
    Navigator::new(maze).from(pos).down(true);
    assert!(
        maze.walls(pos)
            .iter()
            .filter(|wall| maze.is_open((pos, wall)))
            .count() == 1
    );
    assert!(
        maze.walls(next)
            .iter()
            .filter(|wall| maze.is_open((next, wall)))
            .count() == 1
    );
}

maze_test!(can_open, can_open_test);


fn can_close(maze: &mut Maze) {
    let pos = (0, 0);
    let next = (0, 1);
    Navigator::new(maze).from(pos).down(true).up(false);
    assert!(
        maze.walls(pos)
            .iter()
            .filter(|wall| maze.is_open((pos, wall)))
            .count() == 0
    );
    assert!(
        maze.walls(next)
            .iter()
            .filter(|wall| maze.is_open((next, wall)))
            .count() == 0
    );
}

maze_test!(can_close, can_close_test);


fn walls_correct(maze: &mut Maze) {
    let walls = maze.walls((0, 1));
    assert_eq!(
        walls
            .iter()
            .cloned()
            .collect::<HashSet<&wall::Wall>>()
            .len(),
        walls.len()
    );
}

maze_test!(walls_correct, walls_correct_test);


fn connected_correct(maze: &mut Maze) {
    for x in 0..maze.width() {
        for y in 0..maze.height() {
            let pos = (x as isize, y as isize);
            assert!(maze.connected(pos, pos))
        }
    }

    let pos1 = (1, 1);
    for wall in maze.walls(pos1) {
        let pos2 = (pos1.1 + wall.dir.0, pos1.1 + wall.dir.1);
        assert!(!maze.connected(pos1, pos2));
        maze.open((pos1, wall));
        assert!(maze.connected(pos1, pos2));
    }
}

maze_test!(connected_correct, connected_correct_test);


fn walk_disconnected(maze: &mut Maze) {
    assert!(maze.walk((0, 0), (0, 1)).is_none());
}

maze_test!(walk_disconnected, walk_disconnected_test);


fn walk_same(maze: &mut Maze) {
    let from = (0, 0);
    let to = (0, 0);
    let expected = vec![(0, 0)];
    assert!(
        maze.walk(from, to).unwrap().collect::<Vec<matrix::Pos>>() == expected
    );
}

maze_test!(walk_same, walk_same_test);


fn walk_simple(maze: &mut Maze) {
    Navigator::new(maze).from((0, 0)).down(true);

    let from = (0, 0);
    let to = (0, 1);
    let expected = vec![(0, 0), (0, 1)];
    assert!(
        maze.walk(from, to).unwrap().collect::<Vec<matrix::Pos>>() == expected
    );
}

maze_test!(walk_simple, walk_simple_test);


fn walk_shortest(maze: &mut Maze) {
    Navigator::new(maze)
        .from((0, 0))
        .down(true)
        .down(true)
        .down(true)
        .right(true)
        .right(true)
        .up(true);

    let from = (0, 0);
    let to = (1, 3);
    let expected = vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 3)];
    assert!(
        maze.walk(from, to).unwrap().collect::<Vec<matrix::Pos>>() == expected
    );
}

maze_test!(walk_shortest, walk_shortest_test);


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
        self.navigate(|wall| wall.dir.1 < 0, open)
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
        self.navigate(|wall| wall.dir.1 > 0, open)
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
        self.navigate(|wall| wall.dir.0 < 0, open)
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
        self.navigate(|wall| wall.dir.0 > 0, open)
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

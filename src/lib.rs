pub mod matrix;
pub mod room;
pub mod walker;

#[macro_use]
pub mod wall;

mod open_set;


/// A maze contains rooms and has methods for managing paths and doors.
pub trait Maze {
    /// Returns the width of the maze.
    ///
    /// This is short hand for `self.rooms().width()`.
    fn width(&self) -> usize {
        self.rooms().width
    }

    /// Returns the height of the maze.
    ///
    /// This is short hand for `self.rooms().height()`.
    fn height(&self) -> usize {
        self.rooms().height
    }

    /// Returns whether a specified wall is open.
    ///
    /// # Arguments
    /// * `pos` - The room position.
    /// * `wall` - The wall to check.
    fn is_open(&self, pos: matrix::Pos, wall: &'static wall::Wall) -> bool {
        match self.rooms().get(pos) {
            Some(room) => room.is_open(wall),
            None => false,
        }
    }

    /// Sets whether a wall is open.
    ///
    /// # Arguments
    /// * `pos` - The room position.
    /// * `wall` - The wall to modify.
    /// * `value` - Whether to open the wall.
    fn set_open(&mut self,
                pos: matrix::Pos,
                wall: &'static wall::Wall,
                value: bool) {
        // First modify the requested wall...
        if let Some(room) = self.rooms_mut().get_mut(pos) {
            room.set_open(wall, value);
        }

        // ..and then sync the value on the back
        let (other_pos, other_wall) = self.back(pos, wall);
        if let Some(other_room) = self.rooms_mut().get_mut(other_pos) {
            other_room.set_open(other_wall, value);
        }
    }

    /// Opens a wall.
    ///
    /// # Arguments
    /// * `pos` - The room position.
    /// * `wall` - The wall to open.
    fn open(&mut self, pos: matrix::Pos, wall: &'static wall::Wall) {
        self.set_open(pos, wall, true);
    }

    /// Closes a wall.
    ///
    /// # Arguments
    /// * `pos` - The room position.
    /// * `wall` - The wall to close.
    fn close(&mut self, pos: matrix::Pos, wall: &'static wall::Wall) {
        self.set_open(pos, wall, false);
    }

    /// Returns the back of a wall.
    ///
    /// The back is the other side of the wall, located in a neighbouring room.
    ///
    /// # Arguments
    /// * `pos` - The room position.
    /// * `wall` - The wall.
    fn back(&self,
            pos: matrix::Pos,
            wall: &'static wall::Wall)
            -> (matrix::Pos, &'static wall::Wall) {
        let other = (pos.0 + wall.dx, pos.1 + wall.dy);
        (other, self.opposite(other, wall).unwrap())
    }

    /// Walks from `from` to `to` along the sortest path.
    ///
    /// If the rooms are connected, the return value will iterate over the
    /// minimal set of rooms required to pass through to get from start to
    /// finish, including `from` and ` to`.
    ///
    /// # Arguments
    /// * `from` - The starting position.
    /// * `to` - The desired goal.
    fn walk(&self,
            from: matrix::Pos,
            to: matrix::Pos)
            -> Option<walker::Walker> {
        // Reverse the positions to return the rooms in correct order
        let (start, end) = (to, from);

        /// The heuristic for a room position
        let h =
            |pos: matrix::Pos| (pos.0 - end.0).abs() + (pos.1 - end.1).abs();

        // The room positions already evaluated
        let mut closed_set = std::collections::HashSet::new();

        // The room positions pending evaluation and their cost
        let mut open_set = open_set::OpenSet::new();
        open_set.push(std::isize::MAX, start);

        // The cost from start to a room along the best known path
        let mut g_score = std::collections::HashMap::new();
        g_score.insert(start, 0isize);

        // The estimated cost from start to end through a room
        let mut f_score = std::collections::HashMap::new();
        f_score.insert(start, h(start));

        // The room from which we entered a room; when we reach the end, we use
        // this to backtrack to the start
        let mut came_from = std::collections::HashMap::new();

        while let Some(current) = open_set.pop() {
            // Have we reached the target?
            if current == end {
                return Some(walker::Walker::new(current, came_from));
            }

            closed_set.insert(current);
            for wall in self.walls(current) {
                // Ignore closed walls
                if !self.is_open(current, wall) {
                    continue;
                }

                // Find the next room, and continue if we have already evaluated
                // it, or it is outside of the maze
                let (next, _) = self.back(current, wall);
                if !self.rooms().is_inside(next) || closed_set.contains(&next) {
                    continue;
                }

                // The cost to get to this room is one more that the room from
                // which we came
                let g = g_score.get(&current).unwrap() + 1;
                let f = g + h(next);

                if !open_set.contains(current) ||
                   g < *g_score.get(&current).unwrap() {
                    came_from.insert(next, current);
                    g_score.insert(next, g);
                    f_score.insert(next, f);

                    if !open_set.contains(current) {
                        open_set.push(f, next);
                    }
                }
            }
        }

        None
    }

    /// Returns the opposite of a wall.
    ///
    /// The opposite is the wall located on the opposite side of the room. For
    /// mazes with rooms with an odd number of walls, there is no opposite wall.
    ///
    /// # Arguments
    /// * `pos` - The room position.
    /// * `wall` - The wall.
    fn opposite(&self,
                pos: matrix::Pos,
                wall: &'static wall::Wall)
                -> Option<&'static wall::Wall>;

    /// Returns all walls of a specific room.
    ///
    /// # Arguments
    /// * `pos` - The room position.
    fn walls(&self, pos: matrix::Pos) -> &'static [&'static wall::Wall];

    /// Retrieves a reference to the underlying rooms.
    fn rooms(&self) -> &room::Rooms;

    /// Retrieves a mutable reference to the underlying rooms.
    fn rooms_mut(&mut self) -> &mut room::Rooms;
}


pub mod quad;


#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;


    /// Creates a test function that runs the tests for all known types of
    /// mazes.
    macro_rules! maze_test {
        ($test_function:ident, $name:ident) => {
            #[test]
            fn $name() {
                let width = 10;
                let height = 5;

                $test_function(&mut quad::Maze::new(width, height));
            }
        }
    }


    fn is_inside_correct(maze: &mut Maze) {
        assert!(maze.rooms().is_inside((0, 0)));
        assert!(maze.rooms()
            .is_inside((maze.width() as isize - 1,
                        maze.height() as isize - 1)));
        assert!(!maze.rooms().is_inside((-1, -1)));
        assert!(!maze.rooms()
            .is_inside((maze.width() as isize, maze.height() as isize)));
    }

    maze_test!(is_inside_correct, is_inside_correct_test);


    fn can_open(maze: &mut Maze) {
        let pos = (0, 0);
        let next = (0, 1);
        Navigator::new(maze)
            .from(pos)
            .down(true);
        assert!(maze.walls(pos)
            .iter()
            .filter(|wall| maze.is_open(pos, wall))
            .count() == 1);
        assert!(maze.walls(next)
            .iter()
            .filter(|wall| maze.is_open(next, wall))
            .count() == 1);
    }

    maze_test!(can_open, can_open_test);


    fn can_close(maze: &mut Maze) {
        let pos = (0, 0);
        let next = (0, 1);
        Navigator::new(maze)
            .from(pos)
            .down(true)
            .up(false);
        assert!(maze.walls(pos)
            .iter()
            .filter(|wall| maze.is_open(pos, wall))
            .count() == 0);
        assert!(maze.walls(next)
            .iter()
            .filter(|wall| maze.is_open(next, wall))
            .count() == 0);
    }

    maze_test!(can_close, can_close_test);


    fn walls_correct(maze: &mut Maze) {
        let walls = maze.walls((0, 1));
        assert_eq!(walls.iter()
                       .cloned()
                       .collect::<HashSet<&wall::Wall>>()
                       .len(),
                   walls.len());
    }

    maze_test!(walls_correct, walls_correct_test);


    fn walk_disconnected(maze: &mut Maze) {
        assert!(maze.walk((0, 0), (0, 1)).is_none());
    }

    maze_test!(walk_disconnected, walk_disconnected_test);


    fn walk_same(maze: &mut Maze) {
        let from = (0, 0);
        let to = (0, 0);
        let expected = vec![(0, 0)];
        assert!(maze.walk(from, to).unwrap().collect::<Vec<matrix::Pos>>() ==
                expected);
    }

    maze_test!(walk_same, walk_same_test);


    fn walk_simple(maze: &mut Maze) {
        Navigator::new(maze)
            .from((0, 0))
            .down(true);

        let from = (0, 0);
        let to = (0, 1);
        let expected = vec![(0, 0), (0, 1)];
        assert!(maze.walk(from, to).unwrap().collect::<Vec<matrix::Pos>>() ==
                expected);
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
        assert!(maze.walk(from, to).unwrap().collect::<Vec<matrix::Pos>>() ==
                expected);
    }

    maze_test!(walk_shortest, walk_shortest_test);


    /// A navigator through a maze.
    ///
    /// This struct provides utility methods to open and close doors based on
    /// directions.
    struct Navigator<'a> {
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
            self.navigate(|wall| wall.dy < 0, open)
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
            self.navigate(|wall| wall.dy > 0, open)
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
            self.navigate(|wall| wall.dx < 0, open)
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
            self.navigate(|wall| wall.dx > 0, open)
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
        fn navigate<'b, P>(&'b mut self,
                           predicate: P,
                           open: bool)
                           -> &'b mut Self
            where for<'r> P: FnMut(&'r &&wall::Wall) -> bool
        {
            let wall = self.maze
                .walls(self.pos)
                .iter()
                .filter(predicate)
                .next()
                .unwrap();
            self.maze.set_open(self.pos, wall, open);
            self.pos = (self.pos.0 + wall.dx, self.pos.1 + wall.dy);
            self
        }
    }
}

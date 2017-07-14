extern crate ndarray;

pub mod room;
pub mod walker;

#[macro_use]
pub mod wall;

mod open_set;


/// A room position.
///
/// The position is not an attribute of a [room](trait.Room.html), but a room
/// can be accessed from a [maze](../struct.Maze.html).
pub type Pos = (isize, isize);


/// A matrix of rooms.
///
/// A room matrix has a width and a height, and rooms can be addressed by
// position.
trait Rooms<T>
    where T: Clone + Default
{
    /// The number of rooms across the maze, horizontally.
    fn width(&self) -> usize;

    /// The number of rooms across the maze, vertically.
    fn height(&self) -> usize;

    /// Determines whether a position is inside of the maze.
    ///
    /// # Arguments
    /// * `pos` - The room position.
    fn is_inside(&self, pos: Pos) -> bool {
        pos.0 >= 0 && pos.1 >= 0 && pos.0 < self.width() as isize &&
        pos.1 < self.height() as isize
    }


    /// Retrieves a reference to the room at a specific position if it exists.
    ///
    /// # Arguments
    /// * `pos` - The room position.
    fn get(&self, pos: Pos) -> Option<&room::Room<T>>;

    /// Retrieves a mutable reference to the room at a specific position if it
    /// exists.
    ///
    /// # Arguments
    /// * `pos` - The room position.
    fn get_mut(&mut self, pos: Pos) -> Option<&mut room::Room<T>>;
}


/// A maze contains rooms and has methods for managing paths and doors.
trait Maze<T>
    where T: Clone + Default
{
    /// Returns whether a specified wall is open.
    ///
    /// # Arguments
    /// * `pos` - The room position.
    /// * `wall` - The wall to check.
    fn is_open(&self, pos: Pos, wall: &'static wall::Wall) -> bool {
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
    fn set_open(&mut self, pos: Pos, wall: &'static wall::Wall, value: bool) {
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
    fn open(&mut self, pos: Pos, wall: &'static wall::Wall) {
        self.set_open(pos, wall, true);
    }

    /// Closes a wall.
    ///
    /// # Arguments
    /// * `pos` - The room position.
    /// * `wall` - The wall to close.
    fn close(&mut self, pos: Pos, wall: &'static wall::Wall) {
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
            pos: Pos,
            wall: &'static wall::Wall)
            -> (Pos, &'static wall::Wall) {
        let other = (pos.0 + wall.dx, pos.1 + wall.dy);
        (other, self.opposite(other, wall).unwrap())
    }

    /// Walks from `from` to `to` along the sortest path.
    ///
    /// If the rooms are connected, the returns value will iterate over the
    /// minimal set of rooms required to pass through to get from start to
    /// finish, including `from` and ` to`.
    ///
    /// # Arguments
    /// * `from` - The starting position.
    /// * `to` - The desired goal.
    fn walk(&self, from: Pos, to: Pos) -> Option<walker::Walker> {
        // Reverse the positions to return the rooms in correct order
        let (start, end) = (to, from);

        /// The heuristic for a room position
        let h = |pos: Pos| (pos.0 - end.0).abs() + (pos.1 - end.1).abs();

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
                pos: Pos,
                wall: &'static wall::Wall)
                -> Option<&'static wall::Wall>;

    /// Returns all walls of a specific room.
    ///
    /// # Arguments
    /// * `pos` - The room position.
    fn walls(&self, pos: Pos) -> &'static [&'static wall::Wall];

    /// Retrieves a reference to the underlying rooms.
    fn rooms(&self) -> &Rooms<T>;

    /// Retrieves a mutable reference to the underlying rooms.
    fn rooms_mut(&mut self) -> &mut Rooms<T>;
}


pub mod ndarray_rooms;


#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use Maze;
    use Pos;
    use Rooms;
    use ndarray_rooms;
    use wall;


    define_walls! {
        UP = { dx: 0, dy: -1 },
        LEFT = { dx: -1, dy: 0},
        DOWN = { dx: 0, dy: 1},
        RIGHT = { dx: 1, dy: 0}
    }


    pub struct TestMaze {
        rooms: ndarray_rooms::Rooms<u32>,
    }

    impl TestMaze {
        pub fn new(width: usize, height: usize) -> TestMaze {
            TestMaze { rooms: ndarray_rooms::Rooms::new(width, height) }
        }
    }

    impl ::Maze<u32> for TestMaze {
        #[allow(unused_variables)]
        fn opposite(&self,
                    pos: Pos,
                    wall: &'static ::wall::Wall)
                    -> Option<&'static ::wall::Wall> {
            Some(&walls::ALL[(wall.index + walls::ALL.len() / 2) %
                             walls::ALL.len()])
        }

        #[allow(unused_variables)]
        fn walls(&self, pos: Pos) -> &'static [&'static wall::Wall] {
            &walls::ALL
        }

        fn rooms(&self) -> &Rooms<u32> {
            &self.rooms
        }

        fn rooms_mut(&mut self) -> &mut Rooms<u32> {
            &mut self.rooms
        }
    }


    /// Creates a test function that runs the tests for all known types of mazes.
    macro_rules! maze_test {
        ($test_function:ident, $name:ident) => {
            #[test]
            fn $name() {
                let width = 10;
                let height = 5;

                $test_function(&mut TestMaze::new(width, height));
            }
        }
    }


    fn is_inside_correct<T: Clone + Default>(maze: &mut Maze<T>) {
        assert!(maze.rooms().is_inside((0, 0)));
        assert!(maze.rooms()
            .is_inside((maze.rooms().width() as isize - 1,
                        maze.rooms().height() as isize - 1)));
        assert!(!maze.rooms().is_inside((-1, -1)));
        assert!(!maze.rooms().is_inside((maze.rooms().width() as isize,
                                         maze.rooms().height() as isize)));
    }

    maze_test!(is_inside_correct, is_inside_correct_test);


    fn can_open<T: Clone + Default>(maze: &mut Maze<T>) {
        maze.open((0, 0), &walls::DOWN);
        assert!(maze.is_open((0, 0), &walls::DOWN));
        assert!(maze.is_open((0, 1), &walls::UP));
    }

    maze_test!(can_open, can_open_test);


    fn can_close<T: Clone + Default>(maze: &mut Maze<T>) {
        maze.open((0, 0), &walls::DOWN);
        maze.close((0, 1), &walls::UP);
        assert!(!maze.is_open((0, 0), &walls::DOWN));
        assert!(!maze.is_open((0, 1), &walls::UP));
    }

    maze_test!(can_close, can_close_test);


    fn walls_correct<T: Clone + Default>(maze: &mut Maze<T>) {
        let walls = maze.walls((0, 1));
        assert_eq!(walls.iter()
                       .cloned()
                       .collect::<HashSet<&wall::Wall>>()
                       .len(),
                   walls.len());
    }

    maze_test!(walls_correct, walls_correct_test);


    fn walk_disconnected<T: Clone + Default>(maze: &mut Maze<T>) {
        assert!(maze.walk((0, 0), (0, 1)).is_none());
    }

    maze_test!(walk_disconnected, walk_disconnected_test);


    fn walk_same<T: Clone + Default>(maze: &mut Maze<T>) {
        let from = (0, 0);
        let to = (0, 0);
        let expected = vec![(0, 0)];
        assert!(maze.walk(from, to).unwrap().collect::<Vec<Pos>>() == expected);
    }

    maze_test!(walk_same, walk_same_test);


    fn walk_simple<T: Clone + Default>(maze: &mut Maze<T>) {
        maze.open((0, 0), &walls::DOWN);

        let from = (0, 0);
        let to = (0, 1);
        let expected = vec![(0, 0), (0, 1)];
        assert!(maze.walk(from, to).unwrap().collect::<Vec<Pos>>() == expected);
    }

    maze_test!(walk_simple, walk_simple_test);


    fn walk_shortest<T: Clone + Default>(maze: &mut Maze<T>) {
        maze.open((0, 0), &walls::DOWN);
        maze.open((0, 1), &walls::DOWN);
        maze.open((0, 2), &walls::DOWN);
        maze.open((0, 2), &walls::RIGHT);
        maze.open((0, 3), &walls::RIGHT);
        maze.open((1, 3), &walls::UP);

        let from = (0, 0);
        let to = (1, 3);
        let expected = vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 3)];
        assert!(maze.walk(from, to).unwrap().collect::<Vec<Pos>>() == expected);
    }

    maze_test!(walk_shortest, walk_shortest_test);
}

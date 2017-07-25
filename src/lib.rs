pub mod matrix;
pub mod room;

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


#[cfg(test)]
#[macro_use]
mod tests;

pub mod quad;
pub mod walker;

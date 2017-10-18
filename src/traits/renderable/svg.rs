use svg;
use svg::node::element::path::{Command, Position};

use Maze;
use WallPos;

use matrix;
use physical;
use wall;


pub trait ToPath {
    /// Generates an _SVG path d_ attribute value.
    fn to_path_d(&self) -> svg::node::element::path::Data;
}


impl<'a> ToPath for Maze + 'a {
    fn to_path_d(&self) -> svg::node::element::path::Data {
        let mut commands = Vec::new();
        let mut visited = Visitor::new(self);

        // While a non-visited wall still exists, walk along it
        while let Some((next_pos, next_wall)) = visited.next_wall() {
            for (i, (from, to)) in self.follow_wall((next_pos, next_wall))
                .enumerate()
            {
                // Ensure the wall has not been visited before
                if visited.visited(from) {
                    break;
                } else {
                    visited.visit(from);
                }

                // For the first wall, we need to move to the corner furthest
                // from the second wall, or just any corner if this is a
                // one-wall line
                if i == 0 {
                    if let Some(next) = to {
                        let (_, pos) = corners(self, from, center(self, next));
                        commands.push(Operation::Move(pos));
                    } else {
                        let (pos, _) = self.corners(from);
                        commands.push(Operation::Move(pos));
                    }
                }

                // Draw a line from the previous point to the point of the
                // current wall furthest away
                let (_, pos) =
                    corners(self, from, commands.last().unwrap().pos());
                commands.push(Operation::Line(pos));

                // If the next room is outside of the maze, break
                if to.map(|(pos, _)| !self.rooms().is_inside(pos)).unwrap_or(
                    false,
                )
                {
                    break;
                }
            }
        }

        svg::node::element::path::Data::from(
            commands
                .iter()
                .map(|c| c.command())
                .collect::<Vec<Command>>(),
        )
    }
}


/// A visitor for wall positions.
///
/// This struct provides means to visit all wall positions of a maze.
struct Visitor<'a> {
    /// The maze whose walls are being visited.
    maze: &'a Maze,

    /// The visited walls.
    walls: matrix::Matrix<wall::Mask>,

    /// The current room.
    index: usize,
}


impl<'a> Visitor<'a> {
    pub fn new(maze: &'a Maze) -> Self {
        Self {
            maze: maze,
            walls: matrix::Matrix::new(maze.width(), maze.height()),
            index: 0,
        }
    }

    fn visit(&mut self, wall_pos: WallPos) {
        if let Some(mut mask) = self.walls.get_mut(wall_pos.0) {
            *mask = *mask | (1 << wall_pos.1.index);
        }

        let back = self.maze.back(wall_pos);
        if let Some(mut back_mask) = self.walls.get_mut(back.0) {
            *back_mask = *back_mask | (1 << back.1.index);
        }
    }

    fn visited(&self, wall_pos: WallPos) -> bool {
        if let Some(mask) = self.walls.get(wall_pos.0) {
            (mask & (1 << wall_pos.1.index)) != 0
        } else {
            false
        }
    }

    /// Returns the next non-visited wall.
    fn next_wall(&mut self) -> Option<WallPos> {
        while let Some(pos) = self.pos() {
            if let Some(next) = self.maze.walls(pos)
                .iter()

                // Keep only closed walls that have not yet been drawn
                .filter(|&w| !self.maze.is_open((pos, w)))
                .filter(|&w| !self.visited((pos, *w)))

                .map(|&w| (pos, w))
                .next()
            {
                return Some(next);
            } else {
                self.index = self.index + 1;
            }
        }

        None
    }

    /// Returns the current room.
    ///
    /// This function transforms the index to a room position.
    fn pos(&self) -> Option<matrix::Pos> {
        while self.index < self.maze.width() * self.maze.height() {
            let pos = (
                (self.index % self.maze.width()) as isize,
                (self.index / self.maze.width()) as isize,
            );
            return Some(pos);
        }

        None
    }
}


enum Operation {
    Move(physical::Pos),
    Line(physical::Pos),
}


impl Operation {
    fn command(&self) -> Command {
        match *self {
            Operation::Move(pos) => {
                Command::Move(Position::Absolute, (pos.0, pos.1).into())
            }
            Operation::Line(pos) => {
                Command::Line(Position::Absolute, (pos.0, pos.1).into())
            }
        }
    }

    fn pos(&self) -> physical::Pos {
        match self {
            &Operation::Move(pos) |
            &Operation::Line(pos) => pos,
        }
    }
}


/// Returns the center of a wall.
///
/// The center of a wall is the point between its corners.
///
/// # Arguments
/// * `wall_pos` - The wall position.
fn center(maze: &Maze, wall_pos: WallPos) -> physical::Pos {
    let (corner1, corner2) = maze.corners(wall_pos);
    ((corner1.0 + corner2.0) / 2.0, (corner1.1 + corner2.1) / 2.0)
}


/// Returns the physical positions of the two corners of a wall ordered by
/// distance to another wall.
///
/// # Arguments
/// * `from` - The wall position.
/// * `to` - The next wall position from which distances are calculated.
fn corners(
    maze: &Maze,
    from: WallPos,
    origin: physical::Pos,
) -> (physical::Pos, physical::Pos) {
    let (pos1, pos2) = maze.corners(from);
    let d1 = (pos1.0 - origin.0).powi(2) + (pos1.1 - origin.1).powi(2);
    let d2 = (pos2.0 - origin.0).powi(2) + (pos2.1 - origin.1).powi(2);

    if d1 < d2 { (pos1, pos2) } else { (pos2, pos1) }
}

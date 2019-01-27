pub use maze::physical;

use crate::*;

/// Our views.
pub enum View {
    /// A view centered above a specific physical position.
    FromAbove {
        /// The centre position.
        pos: physical::Pos,

        /// The zoom level.
        zoom: f32,
    },
}

/// A viewport.
#[derive(Debug)]
pub struct Viewport {
    /// The top left corner.
    pub top_left: maze::physical::Pos,

    /// The bottom right corner.
    pub bottom_right: maze::physical::Pos,
}

impl Viewport {
    /// Creates a new viewport centered around `center` with the spcified
    /// dimensions.
    ///
    /// # Arguments
    /// *  `center` - The centre of the viewport.
    /// *  `width` - The width.
    /// *  `height` - The height.
    pub fn new(center: maze::physical::Pos, width: f32, height: f32) -> Self {
        Self {
            top_left: physical::Pos {
                x: center.x - width * 0.5,
                y: center.y - height * 0.5,
            },
            bottom_right: physical::Pos {
                x: center.x + width * 0.5,
                y: center.y + height * 0.5,
            },
        }
    }

    /// Determines whether this viewport contains a point.
    ///
    /// The right and bottom edges are not considered part of the viewport.
    ///
    /// # Argument
    /// *  `pos` - The point to check.
    pub fn contains(&self, pos: maze::physical::Pos) -> bool {
        pos.x >= self.top_left.x
            && pos.y >= self.top_left.y
            && pos.x < self.bottom_right.x
            && pos.y < self.bottom_right.y
    }

    /// Determines the cente of this viewport.
    pub fn center(&self) -> physical::Pos {
        physical::Pos {
            x: (self.top_left.x + self.bottom_right.x) / 2.0,
            y: (self.top_left.y + self.bottom_right.y) / 2.0,
        }
    }

    /// Lists all room positions visible for this viewport.
    ///
    /// A room position is considered to be visible if at least one corner of
    /// the corresponding room is inside the viewport.
    ///
    /// # Arguments
    /// *  `maze` - The maze whose rooms to list.
    pub fn room_positions(&self, maze: &maze::Maze) -> Vec<matrix::Pos> {
        let mut result = vec![];
        let room_pos = maze.room_at(self.center());

        // List all rooms with a vertical or horisontal distance d from the
        // centre room
        let mut d = 0;
        loop {
            // Find all visible rooms at distance d
            let mut room_positions = (room_pos.col - d..room_pos.col + d + 1)
                .flat_map(move |col| {
                    (room_pos.row - d..room_pos.row + d + 1)
                        .map(move |row| matrix::Pos { col, row })
                })
                .filter(|&matrix::Pos { col, row }| {
                    col + d == room_pos.col
                        || row + d == room_pos.row
                        || col - d == room_pos.col
                        || row - d == room_pos.row
                })
                // Keep only rooms within our current viewport
                .filter(|&room_pos| {
                    maze.walls(room_pos)
                        .iter()
                        .map(|&wall| maze.corners((room_pos, wall)).0)
                        .any(|corner| self.contains(corner))
                })
                .peekable();

            if room_positions.peek().is_some() {
                result.extend(room_positions);
                d += 1;
            } else {
                break;
            }
        }

        return result;
    }
}

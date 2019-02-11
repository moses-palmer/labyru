use crate::matrix;

use crate::WallPos;

/// A physical position.
#[derive(Clone, Copy, Debug)]
pub struct Pos {
    /// The X coordinate.
    pub x: f32,

    /// The Y coordinate.
    pub y: f32,
}

/// An object that has some "physical" properties.
pub trait Physical {
    /// Returns the "physical" centre of a matrix position.
    ///
    /// # Arguments
    /// * `pos` - The matrix position.
    fn center(&self, pos: matrix::Pos) -> Pos;

    /// Returns the matrix position whose centre is closest to a "physical"
    /// position.
    ///
    /// The position returned may not correspond to an actual room; it may lie
    /// outside of the maze.
    ///
    /// # Arguments
    /// * `pos` - The physical position.
    fn room_at(&self, pos: Pos) -> matrix::Pos;

    /// Returns the "physical" positions of the two corners of a wall.
    ///
    /// # Arguments
    /// * `pos` - The matrix position.
    /// * `wall` - The wall.
    fn corners(&self, wall_pos: WallPos) -> (Pos, Pos) {
        let center = self.center(wall_pos.0);
        (
            Pos {
                x: center.x + wall_pos.1.span.0.cos(),
                y: center.y + wall_pos.1.span.0.sin(),
            },
            Pos {
                x: center.x + wall_pos.1.span.1.cos(),
                y: center.y + wall_pos.1.span.1.sin(),
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;
    use crate::*;

    maze_test!(
        room_at,
        fn test(maze: &mut Maze) {
            let (left, top, width, height) = maze.viewbox();
            let Pos { x: min_x, y: min_y } = maze.center(matrix_pos(0, 0));
            let Pos { x: max_x, y: max_y } = maze.center(matrix_pos(
                maze.width() as isize - 1,
                maze.height() as isize - 1,
            ));
            let xres = 100usize;
            let yres = 100usize;
            for x in 0..xres {
                for y in 0..yres {
                    let pos = Pos {
                        x: x as f32 / (xres as f32 * width + left),
                        y: y as f32 / (yres as f32 * height + top),
                    };

                    // Should this position be inside the maze?
                    let assume_inside = true
                        && pos.x >= min_x
                        && pos.x <= max_x
                        && pos.y >= min_y
                        && pos.y <= max_y;

                    // Ignore rooms outside of the maze since we use
                    // maze.rooms().positions() below
                    let actual = maze.room_at(pos);
                    if !maze.rooms().is_inside(actual) && !assume_inside {
                        continue;
                    }

                    let mut positions = maze
                        .rooms()
                        .positions()
                        .map(|matrix_pos| (maze.center(matrix_pos), matrix_pos))
                        .map(|(physical_pos, matrix_pos)| {
                            (distance(pos, physical_pos), matrix_pos)
                        })
                        .collect::<Vec<_>>();
                    positions.sort_by_key(|&(k, _)| k);

                    let (_, expected) = positions[0];
                    assert_eq!(expected, actual);
                }
            }
        }
    );

    /// Calculates an integral distance value between two points.
    ///
    /// # Arguments
    /// * `pos1` - The first point.
    /// * `pos2` - The second point.
    fn distance(pos1: physical::Pos, pos2: physical::Pos) -> u64 {
        (10000000000.0 * true_distance(pos1, pos2)) as u64
    }

    /// Calculates the actual distance value between two points.
    ///
    /// # Arguments
    /// * `pos1` - The first point.
    /// * `pos2` - The second point.
    fn true_distance(pos1: physical::Pos, pos2: physical::Pos) -> f32 {
        let dx = pos1.x - pos2.x;
        let dy = pos1.y - pos2.y;
        (dx * dx + dy * dy).sqrt()
    }
}

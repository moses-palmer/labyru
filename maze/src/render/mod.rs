use std;

use crate::matrix;
use crate::Maze;

impl Maze {
    /// Calculates the _view box_ for an object when rendered.
    ///
    /// The returned tuple _(left, top, width, height)_ is the minimal rectangle
    /// that will contain the walls of the maze.
    pub fn viewbox(&self) -> (f32, f32, f32, f32) {
        let mut window =
            (std::f32::MAX, std::f32::MAX, std::f32::MIN, std::f32::MIN);
        for y in 0..self.height() {
            let lpos = matrix::Pos {
                col: 0,
                row: y as isize,
            };
            let lcenter = self.center(lpos);
            let left = self.walls(lpos).iter().map(|wall| (lcenter, wall));

            let rpos = matrix::Pos {
                col: self.width() as isize - 1,
                row: y as isize,
            };
            let rcenter = self.center(rpos);
            let right = self.walls(rpos).iter().map(|wall| (rcenter, wall));

            window = left
                .chain(right)
                .map(|(center, wall)| {
                    (
                        center.x + f32::cos(wall.span.0),
                        center.y + f32::sin(wall.span.0),
                    )
                })
                .fold(window, |acc, v| {
                    (
                        acc.0.min(v.0),
                        acc.1.min(v.1),
                        acc.2.max(v.0),
                        acc.3.max(v.1),
                    )
                });
        }

        (window.0, window.1, window.2 - window.0, window.3 - window.1)
    }
}

#[cfg(feature = "render-svg")]
pub mod svg;

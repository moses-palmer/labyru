use std::iter;

use maze;
use maze::matrix;

pub use crate::{State, View, GL};

mod from_above;

impl<T> State<T>
where
    T: AsRef<maze::Maze>,
{
    pub fn render(&self, gl: &GL, view: &View) {
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear(
            GL::COLOR_BUFFER_BIT
                | GL::DEPTH_BUFFER_BIT
                | GL::STENCIL_BUFFER_BIT,
        );

        match view {
            View::FromAbove { pos, zoom } => {
                self.render_from_above(gl, *pos, *zoom)
            }
        }
    }

    /// Renders a specific room.
    ///
    /// # Arguments
    /// *  `gl` - The rendering context.
    /// *  `position` - The room to render.
    fn render_room(&self, gl: &GL, position: matrix::Pos) {
        let maze: &maze::Maze = self.maze.as_ref();
        let center = maze.center(position);
        let z_bottom = 0.0f32;
        let z_top = 1.0f32;

        // Create the floor
        // TODO: Implement!
        let floor = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&floor));

        let data = maze.walls(position).iter().enumerate().fold(
            vec![center.x, center.y, z_bottom],
            |mut acc, (i, wall)| {
                if i == 0 {
                    acc.push(center.x + wall.span.0.cos());
                    acc.push(center.y + wall.span.0.sin());
                    acc.push(z_bottom);
                }
                acc.push(center.x + wall.span.1.cos());
                acc.push(center.y + wall.span.1.sin());
                acc.push(z_bottom);
                acc
            },
        );
        gl.buffer_data_with_array_buffer_view(
            GL::ELEMENT_ARRAY_BUFFER,
            &data.into(),
            GL::DYNAMIC_DRAW,
        );
    }
}

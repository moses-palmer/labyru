use maze;
use maze::matrix;
use maze::physical;

use super::*;
use crate::view::Viewport;

impl<T> State<T>
where
    T: AsRef<maze::Maze>,
{
    pub(super) fn render_from_above(
        &self,
        gl: &GL,
        pos: physical::Pos,
        zoom: f32,
    ) {
        let maze: &maze::Maze = self.maze.as_ref();
        let room_pos = maze.room_at(pos);
        let room_center = maze.center(room_pos);
        let offset = (pos.x - room_center.x, pos.y - room_center.y);
        let viewport = viewport(gl, pos, zoom);

        for room_position in viewport.room_positions(maze) {
            // TODO: Translate
            self.render_room(gl, room_pos);
        }
    }
}

/// Calculates the viewport currently visible.
fn viewport(gl: &GL, pos: physical::Pos, zoom: f32) -> Viewport {
    Viewport::new(
        pos,
        gl.drawing_buffer_width() as f32 / zoom,
        gl.drawing_buffer_height() as f32 / zoom,
    )
}

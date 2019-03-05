use maze;
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
        let _offset = (pos.x - room_center.x, pos.y - room_center.y);
        let viewport = viewport(gl, pos, zoom);

        let vert_shader = compile_shader(
            &gl,
            GL::VERTEX_SHADER,
            r#"
                attribute vec4 position;
                void main() {
                    gl_Position = position;
                }
            "#,
        )
        .unwrap();
        let frag_shader = compile_shader(
            &gl,
            GL::FRAGMENT_SHADER,
            r#"
                void main() {
                    gl_FragColor = vec4(0.5, 0.5, 0.5, 1.0);
                }
            "#,
        )
        .unwrap();
        let program =
            link_program(&gl, [vert_shader, frag_shader].iter()).unwrap();
        gl.use_program(Some(&program));

        let room_positions = viewport.room_positions(maze);
        for room_position in room_positions {
            // TODO: Translate
            self.render_room(gl, room_position);
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

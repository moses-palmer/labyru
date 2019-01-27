use super::*;

impl View {
    fn viewport(&self, gl: &GL, pos: physical::Pos, zoom: f32) -> ViewPort {
        Viewport::new(
            pos,
            gl.drawing_buffer_width() as f32 / zoom,
            gl.drawing_buffer_height() as f32 / zoom,
        )
    }
}

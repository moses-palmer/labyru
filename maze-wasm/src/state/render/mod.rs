use js_sys;
use nalgebra;
use wasm_bindgen::JsCast;

use maze;
use maze::matrix;

pub use crate::{State, View, GL};

mod from_above;

impl<T> State<T>
where
    T: AsRef<maze::Maze>,
{
    pub fn render(&self, gl: &GL, view: &View) {
        gl.clear_color(1.0, 1.0, 1.0, 1.0);
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

    fn look_at(
        &self,
        eye: nalgebra::Point3,
        target: nalgebra::Point3,
    ) -> [f32; 16] {
        let view = nalgebra::Isometry3::look_at_rh(
            &eye,
            &target,
            nalgebra::Vector3::y(),
        )
        .to_homogeneous();

        let mut result = [0.; 16];
        result.copy_from_slice(view.as_slice());
        result
    }

    /// Renders a specific room.
    ///
    /// # Arguments
    /// *  `gl` - The rendering context.
    /// *  `position` - The room to render.
    fn render_room(&self, gl: &GL, position: matrix::Pos) {
        let maze: &maze::Maze = self.maze.as_ref();
        let mut center = maze.center(position);
        let z_bottom = 0.0f32;
        let _z_top = 1.0f32;
        center.x = 0.0;
        center.y = 0.0;

        // Create the floor
        // TODO: Implement!

        let data = maze.walls(position).iter().enumerate().fold(
            vec![(center.x, center.y, z_bottom)],
            |mut acc, (i, wall)| {
                if i == 0 {
                    acc.push((
                        center.x + wall.span.0.cos(),
                        center.y + wall.span.0.sin(),
                        z_bottom,
                    ));
                }
                acc.push((
                    center.x + wall.span.1.cos(),
                    center.y + wall.span.1.sin(),
                    z_bottom,
                ));
                acc
            },
        );
        gl.buffer_data_with_array_buffer_view(
            GL::ELEMENT_ARRAY_BUFFER,
            &buffer_data(&data),
            GL::DYNAMIC_DRAW,
        );
        gl.vertex_attrib_pointer_with_i32(attrib, size, GL::FLOAT, false, 0, 0);
        gl.draw_arrays(GL::TRIANGLE_FAN, 0, data.len() as i32);
    }

    fn buffer_data(&self, gl: &GL, inex: u32, source: &[(f32, f32, f32)]) {
        let address = source.as_ptr() as u32 / 4;
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<js_sys::WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(address, address + 3 * source.len() as u32);
        let buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&buffer));
        gl.buffer_data_with_array_buffer_view(
            GL::ARRAY_BUFFER,
            &array,
            GL::STATIC_DRAW,
        );
        gl.vertex_attrib_pointer_with_i32(index, 3, GL::FLOAT, false, 0, 0);
    }
}

pub fn compile_shader(
    context: &GL,
    shader_type: u32,
    source: &str,
) -> Result<web_sys::WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "Unknown error creating shader".into()))
    }
}

pub fn link_program<'a, T: IntoIterator<Item = &'a web_sys::WebGlShader>>(
    context: &GL,
    shaders: T,
) -> Result<web_sys::WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    for shader in shaders {
        context.attach_shader(&program, shader)
    }
    context.link_program(&program);

    if context
        .get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| "Unknown error creating program object".into()))
    }
}

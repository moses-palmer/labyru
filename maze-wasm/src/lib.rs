#![feature(custom_attribute)]

extern crate js_sys;
extern crate maze;
extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;

use maze::initialize;
use maze::initialize::randomized_prim::*;
use maze::matrix;

mod context;
mod state;
mod view;

pub use web_sys::WebGlRenderingContext as GL;

pub use self::state::State;
pub use self::view::View;

/// The application.
#[wasm_bindgen]
pub struct App {
    /// The current state.
    state: State<Box<maze::Maze>>,

    /// The view.
    view: View,
}

#[wasm_bindgen]
impl App {
    /// Creates a new application object.
    ///
    /// # Arguments
    /// *  `canvas_id` - The ID of the canvas element used for drawing.
    /// *  `seed` - The random seed.
    /// *  `walls` - The number of walls per room.
    /// *  `width` - The width, in rooms, of the maze.
    /// *  `height` - The height, in rooms, of the maze.
    ///
    /// # Panics
    /// This method will panic if it fails to create a WebGL context for a
    /// canvas element created for the div, or if the value of `walls` is not
    /// supported.
    #[wasm_bindgen(constructor)]
    pub fn new(seed: u32, walls: u32, width: usize, height: usize) -> Self {
        let mut maze = maze::MazeType::from_num(walls)
            .unwrap()
            .create(width, height);
        maze.randomized_prim(&mut initialize::LFSR::new(seed as u64));

        let view = View::FromAbove {
            pos: maze.center(matrix::Pos { col: 0, row: 0 }),
            zoom: 2.0,
        };

        let state = State { maze };

        Self { state, view }
    }

    /// Renders the current state.
    pub fn render(&self, gl: &GL) {
        self.state.render(gl, &self.view);
    }

    /// Updates the state after rendering.
    pub fn update(&mut self) {}
}

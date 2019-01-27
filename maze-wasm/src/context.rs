use std::ops;
use wasm_bindgen;
use wasm_bindgen::JsCast;
use web_sys;

/// A rendering context.
#[derive(Debug)]
pub struct Context(web_sys::WebGlRenderingContext);

/// A context creation error.
#[derive(Debug)]
pub enum Error {
    /// A reference to the window could not be created.
    Window,

    /// A reference to the document element could not be retrieved.
    Document,

    /// The canvas could not be created.
    Canvas,

    /// The WebGL context could not be retrived.
    Context,

    /// A Javascript error occurred.
    JsError(wasm_bindgen::JsValue),
}

impl Context {
    /// Constructs a new rendering context for a named canvas element.
    ///
    /// # Arguments
    /// *  `canvas_id` - The name of the canvas element.
    pub fn new(canvas_id: &str) -> Result<Self, Error> {
        let window = web_sys::window().ok_or(Error::Window)?;
        let document = window.document().ok_or(Error::Document)?;
        web_sys::console::log_1(&canvas_id.into());

        match document.get_element_by_id(canvas_id) {
            Some(o) => web_sys::console::log_1(&o.into()),
            None => web_sys::console::log_1(&"NONE".into()),
        }

        let canvas: web_sys::HtmlCanvasElement = document
            .get_element_by_id(canvas_id)
            .ok_or(Error::Canvas)?
            .dyn_into()
            .map_err(|e| {
                web_sys::console::log_1(&e.into());
                Error::Canvas
            })?;
        let gl: web_sys::WebGlRenderingContext = canvas
            .get_context("webgl")?
            .ok_or(Error::Context)?
            .dyn_into()
            .map_err(|_| Error::Context)?;
        web_sys::console::log_1(&"context".into());

        Ok(Context(gl))
    }
}

impl ops::Deref for Context {
    type Target = web_sys::WebGlRenderingContext;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Context {
    fn deref_mut(&mut self) -> &mut <Self as ops::Deref>::Target {
        &mut self.0
    }
}

impl From<wasm_bindgen::JsValue> for Error {
    fn from(source: wasm_bindgen::JsValue) -> Self {
        Error::JsError(source)
    }
}

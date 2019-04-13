use rocket::http;
use rocket::request;

use maze;

/// A maze type, convertible from a query string.
pub struct MazeType(maze::Shape);

impl MazeType {
    pub fn create(self, dimensions: super::Dimensions) -> Box<maze::Maze> {
        self.0.create(dimensions.width, dimensions.height)
    }
}

impl<'a> request::FromParam<'a> for MazeType {
    type Error = &'a http::RawStr;

    fn from_param(form_value: &'a http::RawStr) -> Result<Self, Self::Error> {
        match form_value.as_str() {
            "tri" => Ok(Self(maze::Shape::Tri)),
            "quad" => Ok(Self(maze::Shape::Quad)),
            "hex" => Ok(Self(maze::Shape::Hex)),
            _ => Err(form_value),
        }
    }
}

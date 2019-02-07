use rocket::http;
use rocket::request;

use labyru;

/// A maze type, convertible from a query string.
pub struct MazeType(labyru::MazeType);

impl MazeType {
    pub fn create(self, dimensions: super::Dimensions) -> Box<labyru::Maze> {
        self.0.create(dimensions.width, dimensions.height)
    }
}

impl<'a> request::FromParam<'a> for MazeType {
    type Error = &'a http::RawStr;

    fn from_param(form_value: &'a http::RawStr) -> Result<Self, Self::Error> {
        match form_value.as_str() {
            "tri" => Ok(Self(labyru::MazeType::Tri)),
            "quad" => Ok(Self(labyru::MazeType::Quad)),
            "hex" => Ok(Self(labyru::MazeType::Hex)),
            _ => Err(form_value),
        }
    }
}

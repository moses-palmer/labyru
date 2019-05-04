use rocket::http;
use rocket::request;

use maze;

/// A maze type, convertible from a query string.
pub struct MazeType(maze::Shape);

impl MazeType {
    pub fn create(self, dimensions: super::Dimensions) -> maze::Maze {
        self.0.create(dimensions.width, dimensions.height)
    }
}

impl<'a> request::FromParam<'a> for MazeType {
    type Error = String;

    fn from_param(form_value: &'a http::RawStr) -> Result<Self, Self::Error> {
        form_value.as_str().parse().map(MazeType)
    }
}

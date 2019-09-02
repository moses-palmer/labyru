use maze;
use serde::Deserialize;

/// A maze type, convertible from a query string.
#[derive(Deserialize)]
#[serde(transparent)]
pub struct MazeType(maze::Shape);

impl MazeType {
    pub fn create(self, dimensions: super::Dimensions) -> maze::Maze {
        self.0.create(dimensions.width, dimensions.height)
    }
}

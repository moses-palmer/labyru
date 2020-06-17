use serde::Deserialize;

/// A maze type, convertible from a query string.
#[derive(Deserialize)]
#[serde(transparent)]
pub struct MazeType(maze::Shape);

impl MazeType {
    pub fn create<T>(self, dimensions: super::Dimensions) -> maze::Maze<T>
    where
        T: Clone + Copy + Default,
    {
        self.0.create(dimensions.width, dimensions.height)
    }
}

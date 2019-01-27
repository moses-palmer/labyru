use maze;

pub mod render;

pub struct State<T>
where
    T: AsRef<maze::Maze>,
{
    /// The maze.
    pub maze: T,
}

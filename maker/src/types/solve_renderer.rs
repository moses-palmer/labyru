use maze::render::svg::ToPath;

use svg;
use svg::Node;

use crate::types::*;

/// The maze solution.
pub struct SolveRenderer;

impl Renderer for SolveRenderer {
    /// Renders the maze solution.
    ///
    /// # Arguments
    /// * `maze` - The maze.
    /// * `group` - The group to which to add the solution.
    fn render(&self, maze: &maze::Maze, group: &mut svg::node::element::Group) {
        group.append(
            svg::node::element::Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-linecap", "round")
                .set("stroke-linejoin", "round")
                .set("stroke-width", 0.4)
                .set("vector-effect", "non-scaling-stroke")
                .set(
                    "d",
                    maze.walk(
                        maze::matrix::Pos { col: 0, row: 0 },
                        maze::matrix::Pos {
                            col: maze.width() as isize - 1,
                            row: maze.height() as isize - 1,
                        },
                    )
                    .unwrap()
                    .to_path_d(),
                ),
        );
    }
}

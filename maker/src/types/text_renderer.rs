use std::ops;
use std::str::FromStr;

use svg;
use svg::Node;

use maze::physical;
use maze_tools::alphabet;
use maze_tools::focus::*;
use maze_tools::image::Color;

use crate::types::*;

/// A text.
pub struct TextRenderer {
    /// The string to render.
    text: String,
}

impl FromStr for TextRenderer {
    type Err = String;

    /// Converts a string to a string to render.
    ///
    /// The string must be a path.
    fn from_str(s: &str) -> Result<Self, String> {
        Ok(Self { text: s.into() })
    }
}

impl Renderer for TextRenderer {
    /// Applies the text action.
    ///
    /// This action will render a string as background.
    ///
    /// # Arguments
    /// *  `maze` - The maze.
    /// *  `group` - The group to which to add the rooms.
    fn render(&self, maze: &maze::Maze, group: &mut svg::node::element::Group) {
        let (_, _, width, height) = maze.viewbox();
        let columns = (self.text.len() as f32).sqrt().ceil() as usize;
        let rows = (self.text.len() as f32 / columns as f32).ceil() as usize;
        let data = alphabet::default::ALPHABET
            .render(&self.text, columns, 16 * maze.width())
            .map(|(pos, v)| {
                (
                    physical::Pos {
                        x: width * pos.x / columns as f32,
                        y: height * pos.y / rows as f32,
                    },
                    Intermediate::from(v),
                )
            })
            .focus(maze);

        group.append(draw_rooms(maze, |pos| data[pos]));
    }
}

#[derive(Clone, Copy, Default)]
struct Intermediate(f32);

impl From<f32> for Intermediate {
    fn from(source: f32) -> Self {
        Intermediate(source)
    }
}

impl ops::Add<Intermediate> for Intermediate {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Intermediate(self.0 + other.0)
    }
}

impl ops::Div<usize> for Intermediate {
    type Output = Color;

    fn div(self, divisor: usize) -> Self::Output {
        Color {
            red: 0,
            green: 0,
            blue: 0,
            alpha: (255.0 * (1.0 - self.0 / divisor as f32)) as u8,
        }
    }
}

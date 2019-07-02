use std::ops;
use std::str::FromStr;

use image;
use svg;
use svg::Node;

use maze::physical;
use maze_tools::focus::*;
use maze_tools::image::Color;

use crate::types::*;

/// A background image.
pub struct BackgroundRenderer {
    /// The background image.
    pub image: image::RgbImage,
}

impl FromStr for BackgroundRenderer {
    type Err = String;

    /// Converts a string to a background description.
    ///
    /// The string must be a path.
    fn from_str(s: &str) -> Result<Self, String> {
        Ok(Self {
            image: image::open(s)
                .map_err(|_| format!("failed to open {}", s))?
                .to_rgb(),
        })
    }
}

impl Renderer for BackgroundRenderer {
    /// Applies the background action.
    ///
    /// This action will use an image to sample the background colour of rooms.
    ///
    /// # Arguments
    /// * `maze` - The maze.
    /// * `group` - The group to which to add the rooms.
    fn render(&self, maze: &maze::Maze, group: &mut svg::node::element::Group) {
        let (_, _, width, height) = maze.viewbox();
        let (cols, rows) = self.image.dimensions();
        let data = self
            .image
            .enumerate_pixels()
            .map(|(x, y, pixel)| {
                (
                    physical::Pos {
                        x: width * (x as f32 / cols as f32),
                        y: height * (y as f32 / rows as f32),
                    },
                    Intermediate::from(pixel),
                )
            })
            .focus(maze);

        group.append(draw_rooms(maze, |pos| data[pos]));
    }
}

#[derive(Clone, Copy, Default)]
struct Intermediate(u32, u32, u32);

impl<'a, P> From<&'a P> for Intermediate
where
    P: image::Pixel<Subpixel = u8>,
{
    fn from(source: &'a P) -> Self {
        let channels = source.channels();
        Intermediate(
            u32::from(channels[0]),
            u32::from(channels[1]),
            u32::from(channels[2]),
        )
    }
}

impl ops::Add<Intermediate> for Intermediate {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Intermediate(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl ops::Div<usize> for Intermediate {
    type Output = Color;

    fn div(self, divisor: usize) -> Self::Output {
        Color {
            red: (self.0 / divisor as u32) as u8,
            green: (self.1 / divisor as u32) as u8,
            blue: (self.2 / divisor as u32) as u8,
            alpha: 255,
        }
    }
}

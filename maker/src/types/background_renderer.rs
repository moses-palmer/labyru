use std::str::FromStr;

use crate::image;
use crate::svg;
use crate::svg::Node;

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
        let data = image_to_matrix::<_, (u32, (u32, u32, u32))>(
            &self.image,
            maze,
            // Add all pixels inside a room to the cell representing the room
            |matrix, pos, pixel| {
                if maze.rooms().is_inside(pos) {
                    matrix[pos] = (
                        matrix[pos].0 + 1,
                        (
                            (matrix[pos].1).0 + u32::from(pixel[0]),
                            (matrix[pos].1).1 + u32::from(pixel[1]),
                            (matrix[pos].1).2 + u32::from(pixel[2]),
                        ),
                    );
                }
            },
        )
        // Convert the summed colour values to an actual colour
        .map(|value| {
            let (count, pixel) = value;
            Color {
                red: (pixel.0 / (count + 1)) as u8,
                green: (pixel.1 / (count + 1)) as u8,
                blue: (pixel.2 / (count + 1)) as u8,
                alpha: 255,
            }
        });

        group.append(draw_rooms(maze, |pos| data[pos]));
    }
}

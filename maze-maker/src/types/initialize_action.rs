use std::str::FromStr;

use image;
use rand;
use svg;

use super::*;

/// A constant used as multiplier for individual colour values to get an
/// intensity
const D: f32 = 1.0 / 255.0 / 3.0;

/// A masking image.
pub struct InitializeAction {
    /// The mask image.
    pub image: image::RgbImage,

    /// The intensity threshold
    pub threshold: f32,
}

impl FromStr for InitializeAction {
    type Err = String;

    /// Converts a string to an initialise mask description.
    ///
    /// The string must be on the form `path,threshold`, where `path` is the
    /// path to an image and `threshold` is a value between 0 and 1.
    fn from_str(s: &str) -> Result<Self, String> {
        let mut parts = s.split(',').map(str::trim);
        let path = parts
            .next()
            .map(|p| std::path::Path::new(p).to_path_buf())
            .unwrap();

        if let Some(part1) = parts.next() {
            if let Ok(threshold) = part1.parse() {
                Ok(Self {
                    image: image::open(path)
                        .map_err(|_| format!("failed to open {}", s))?
                        .to_rgb(),
                    threshold,
                })
            } else {
                Err(format!("invalid threshold: {}", part1))
            }
        } else {
            Err(format!("invalid mask: {}", s))
        }
    }
}

impl Action for InitializeAction {
    /// Applies the initialise action.
    ///
    /// This action will use the intensity of pixels to determine whether
    /// rooms should be part of the maze.
    ///
    /// # Arguments
    /// * `maze` - The maze.
    /// * `group` - The group to which to add the rooms.
    fn apply(self, maze: &mut maze::Maze, _: &mut svg::node::element::Group) {
        let data = image_to_matrix::<_, f32>(
            &self.image,
            maze,
            // Add all pixel intensities inside a room to the cell representing
            // the room
            |matrix, pos, pixel| {
                if maze.rooms().is_inside(pos) {
                    matrix[pos] += pixel
                        .data
                        .iter()
                        .map(|&p| D * f32::from(p))
                        .sum::<f32>();
                }
            },
        )
        // Convert the summed colour values to an actual colour
        .map(|value| value > self.threshold);

        maze.randomized_prim_filter(&mut rand::weak_rng(), |pos| data[pos]);
    }
}

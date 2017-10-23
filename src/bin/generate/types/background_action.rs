use image;
use svg;
use svg::Node;

use types::*;


/// A background image.
pub struct BackgroundAction {
    /// The path to the background image.
    pub path: std::path::PathBuf,
}


impl Action for BackgroundAction {
    /// Converts a string to a background description.
    ///
    /// The string must be a path.
    fn from_str(s: &str) -> Result<Self, String> {
        Ok(Self { path: std::path::Path::new(s).to_path_buf() })
    }

    /// Applies the background action.
    ///
    /// This action will use an image to sample the background colour of rooms.
    ///
    /// # Arguments
    /// * `maze` - The maze.
    /// * `group` - The group to which to add the rooms.
    #[cfg(feature = "background")]
    fn apply(
        self,
        maze: &mut labyru::Maze,
        group: &mut svg::node::element::Group,
    ) {
        let (left, top, width, height) = maze.viewbox();
        let rgb = image::open(self.path.as_path())
            .expect("unable to open background image")
            .to_rgb();
        let (cols, rows) = rgb.dimensions();
        let data = rgb
            .enumerate_pixels()

            // Add all pixels inside a room to the cell representing the room
            .fold(
                labyru::matrix::Matrix::<(u32, (u32, u32, u32))>::new(
                    maze.width(), maze.height()),
                |mut matrix, (x, y, pixel)| {
                    let physical_pos = (
                        left + width * (x as f32 / cols as f32),
                        top + height * (y as f32 / rows as f32),
                    );
                    let pos = maze.room_at(physical_pos);
                    if maze.rooms().is_inside(pos) {
                        matrix[pos] = (
                            matrix[pos].0 + 1, (
                                (matrix[pos].1).0 + pixel[0] as u32,
                                (matrix[pos].1).1 + pixel[1] as u32,
                                (matrix[pos].1).2 + pixel[2] as u32,
                            ));
                    }

                    matrix
                }
            )

            // Convert the summed colour values to an actual colour
            .map(
                |value| {
                    let (count, pixel) = value;
                    Color {
                        red: (pixel.0 / (count + 1)) as u8,
                        green: (pixel.1 / (count + 1)) as u8,
                        blue: (pixel.2 / (count + 1)) as u8,
                        alpha: 255,
                    }
                }
            );

        group.append(draw_rooms(maze, |pos| data[pos]));
    }

    /// Does nothing
    #[cfg(not(feature = "background"))]
    fn apply(self, _: &mut labyru::Maze, _: &mut svg::node::element::Group) {}
}

use std;
use std::str;

use image;
use rayon;
use svg;

use rayon::prelude::*;
use svg::Node;

use maze;

use maze::matrix::AddableMatrix;

pub mod background_renderer;
pub use self::background_renderer::*;
pub mod break_initializer;
pub use self::break_initializer::*;
pub mod heatmap_renderer;
pub use self::heatmap_renderer::*;
pub mod mask_initializer;
pub use self::mask_initializer::*;
pub mod solve_renderer;
pub use solve_renderer::*;

/// A trait to initialise a maze.
pub trait Initializer {
    /// Initialises a maze.
    ///
    /// # Arguments
    /// *  `maze` - The maze to initialise.
    fn initialize(&self, maze: maze::Maze) -> maze::Maze;
}

impl<T> Initializer for Option<T>
where
    T: Initializer,
{
    fn initialize(&self, maze: maze::Maze) -> maze::Maze {
        if let Some(action) = self {
            action.initialize(maze)
        } else {
            maze
        }
    }
}

/// A trait for rendering a maze.
pub trait Renderer {
    /// Applies this action to a maze and SVG group.
    ///
    /// # Arguments
    /// *  `maze` - The maze.
    /// *  `group` - An SVG group.
    fn render(&self, maze: &maze::Maze, group: &mut svg::node::element::Group);
}

impl<T> Renderer for Option<T>
where
    T: Renderer,
{
    fn render(&self, maze: &maze::Maze, group: &mut svg::node::element::Group) {
        if let Some(action) = self {
            action.render(maze, group);
        }
    }
}

/// A colour.
#[derive(Clone, Copy, Default)]
pub struct Color {
    // The red component.
    pub red: u8,

    // The green component.
    pub green: u8,

    // The blue component.
    pub blue: u8,

    // The alpha component.
    pub alpha: u8,
}

impl Color {
    /// Returns a fully transparent version of this colour.
    fn transparent(self) -> Self {
        Self {
            red: self.red,
            green: self.blue,
            blue: self.blue,
            alpha: 0,
        }
    }

    /// Fades one colour to another.
    ///
    /// # Arguments
    /// * `other` - The other colour.
    /// * `w` - The weight of this colour. If this is `1.0` or greater, `self`
    ///   colour is returned; if this is 0.0 or less, `other` is returned;
    ///   otherwise a linear interpolation between the colours is returned.
    fn fade(self, other: Self, w: f32) -> Color {
        if w >= 1.0 {
            self
        } else if w <= 0.0 {
            other
        } else {
            let n = 1.0 - w;
            Color {
                red: (f32::from(self.red) * w + f32::from(other.red) * n) as u8,
                green: (f32::from(self.green) * w + f32::from(other.green) * n)
                    as u8,
                blue: (f32::from(self.blue) * w + f32::from(other.blue) * n)
                    as u8,
                alpha: (f32::from(self.alpha) * w + f32::from(other.alpha) * n)
                    as u8,
            }
        }
    }
}

impl str::FromStr for Color {
    type Err = String;

    /// Converts a string to a colour.
    ///
    /// This method supports colouts on the form `#RRGGBB` and `#RRGGBBAA`,
    /// where `RR`, `GG`, `BB` and `AA` are the red, green, blue and alpha
    /// components hex encoded.
    ///
    /// # Arguments
    /// * `s` - The string to convert.
    fn from_str(s: &str) -> Result<Color, String> {
        if !s.starts_with('#') || s.len() % 2 == 0 {
            Err(format!("unknown colour value: {}", s))
        } else {
            let data = s
                .bytes()
                // Skip the initial '#'
                .skip(1)
                // Hex decode and create list
                .map(|c| {
                    if c >= b'0' && c <= b'9' {
                        Some(c - b'0')
                    } else if c >= b'A' && c <= b'F' {
                        Some(c - b'A' + 10)
                    } else if c >= b'a' && c <= b'f' {
                        Some(c - b'a' + 10)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                // Join every byte
                .chunks(2)
                .map(|c| {
                    if let (Some(msb), Some(lsb)) = (c[0], c[1]) {
                        Some(msb << 4 | lsb)
                    } else {
                        None
                    }
                })
                // Ensure all values are valid
                .take_while(Option::is_some)
                .map(Option::unwrap)
                .collect::<Vec<_>>();

            match data.len() {
                3 => Ok(Color {
                    red: data[0],
                    green: data[1],
                    blue: data[2],
                    alpha: 255,
                }),
                4 => Ok(Color {
                    red: data[1],
                    green: data[2],
                    blue: data[3],
                    alpha: data[0],
                }),
                _ => Err(format!("invalid colour format: {}", s)),
            }
        }
    }
}

impl ToString for Color {
    /// Converts a colour to a string.
    ///
    /// This method ignores the alpha component.
    fn to_string(&self) -> String {
        format!("#{:02.X}{:02.X}{:02.X}", self.red, self.green, self.blue)
    }
}

/// A type of heat map.
pub enum HeatMapType {
    /// The heat map is generated by traversing vertically.
    Vertical,

    /// The heat map is generated by traversing horisontally.
    Horizontal,

    /// The heat map is generated by travesing from every edge room to the one
    /// on the opposite side.
    Full,
}

impl HeatMapType {
    /// Converts a string to a heat map type.
    ///
    /// # Arguments
    /// * `s` - The string to convert.
    pub fn from_str(s: &str) -> Result<HeatMapType, String> {
        match s {
            "vertical" => Ok(HeatMapType::Vertical),
            "horizontal" => Ok(HeatMapType::Horizontal),
            "full" => Ok(HeatMapType::Full),
            _ => Err(format!("unknown heat map type: {}", s)),
        }
    }

    /// Generates a heat map based on this heat map type.
    ///
    /// # Arguments
    /// * `maze` - The maze for which to generate a heat map.
    pub fn generate(&self, maze: &maze::Maze) -> maze::matrix::Matrix<u32> {
        match *self {
            HeatMapType::Vertical => self.create_heatmap(
                maze,
                (0..maze.width()).map(|col| {
                    (
                        maze::matrix::Pos {
                            col: col as isize,
                            row: 0,
                        },
                        maze::matrix::Pos {
                            col: col as isize,
                            row: maze.height() as isize - 1,
                        },
                    )
                }),
            ),
            HeatMapType::Horizontal => self.create_heatmap(
                maze,
                (0..maze.height()).map(|row| {
                    (
                        maze::matrix::Pos {
                            col: 0,
                            row: row as isize,
                        },
                        maze::matrix::Pos {
                            col: maze.width() as isize - 1,
                            row: row as isize,
                        },
                    )
                }),
            ),
            HeatMapType::Full => self.create_heatmap(
                maze,
                maze.rooms()
                    .positions()
                    .filter(|&pos| pos.col == 0 || pos.row == 0)
                    .map(|pos| {
                        (
                            pos,
                            maze::matrix::Pos {
                                col: maze.width() as isize - 1 - pos.col,
                                row: maze.height() as isize - 1 - pos.row,
                            },
                        )
                    }),
            ),
        }
    }

    /// Generates a heat map for a maze and an iteration of positions.
    ///
    /// # Arguments
    /// * `maze` - The maze for which to generate a heat map.
    /// * `positions` - The positions for which to generate a heat map. These
    ///   will be generated from the heat map type.
    fn create_heatmap<I>(
        &self,
        maze: &maze::Maze,
        positions: I,
    ) -> maze::HeatMap
    where
        I: Iterator<Item = (maze::matrix::Pos, maze::matrix::Pos)>,
    {
        let collected = positions.collect::<Vec<_>>();
        collected
            .chunks(collected.len() / rayon::current_num_threads())
            .collect::<Vec<_>>()
            .par_iter()
            .map(|positions| maze::heatmap(maze, positions.iter().cloned()))
            .reduce(
                || maze::HeatMap::new(maze.width(), maze.height()),
                AddableMatrix::add,
            )
    }
}

/// Draws all rooms of a maze.
///
/// # Arguments
/// * `maze` - The maze to draw.
/// * `colors` - A function determining the colour of a room.
pub fn draw_rooms<F>(maze: &maze::Maze, colors: F) -> svg::node::element::Group
where
    F: Fn(maze::matrix::Pos) -> Color,
{
    let mut group = svg::node::element::Group::new();
    for pos in maze
        .rooms()
        .positions()
        .filter(|pos| maze.rooms()[*pos].visited)
    {
        let color = colors(pos);
        let mut commands = maze
            .walls(pos)
            .iter()
            .enumerate()
            .map(|(i, wall)| {
                let (coords, _) = maze.corners((pos, wall));
                if i == 0 {
                    svg::node::element::path::Command::Move(
                        svg::node::element::path::Position::Absolute,
                        (coords.x, coords.y).into(),
                    )
                } else {
                    svg::node::element::path::Command::Line(
                        svg::node::element::path::Position::Absolute,
                        (coords.x, coords.y).into(),
                    )
                }
            })
            .collect::<Vec<_>>();
        commands.push(svg::node::element::path::Command::Close);

        group.append(
            svg::node::element::Path::new()
                .set("fill", color.to_string())
                .set("fill-opacity", f32::from(color.alpha) / 255.0)
                .set("d", svg::node::element::path::Data::from(commands)),
        );
    }

    group
}

/// Converts an image to a matrix by calling an update function with a pixel
/// and its corresponding matrix position.
///
/// # Arguments
/// *  `image` - The image to convert.
/// *  `maze` - A template maze. This is used to determine which matrix
///    position a pixel corresponds to, and to determine the dimensions of the
///    matrix.
/// *  `update` - The update function.
pub fn image_to_matrix<U, T>(
    image: &image::RgbImage,
    maze: &maze::Maze,
    update: U,
) -> maze::matrix::Matrix<T>
where
    U: Fn(&mut maze::matrix::Matrix<T>, maze::matrix::Pos, &image::Rgb<u8>),
    T: Copy + Default,
{
    let (left, top, width, height) = maze.viewbox();
    let (cols, rows) = image.dimensions();
    image.enumerate_pixels().fold(
        maze::matrix::Matrix::<T>::new(maze.width(), maze.height()),
        |mut matrix, (x, y, pixel)| {
            let physical_pos = maze::physical::Pos {
                x: left + width * (x as f32 / cols as f32),
                y: top + height * (y as f32 / rows as f32),
            };
            let pos = maze.room_at(physical_pos);
            update(&mut matrix, pos, pixel);
            matrix
        },
    )
}

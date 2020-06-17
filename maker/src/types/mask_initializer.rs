use std::ops;
use std::str::FromStr;

use maze::physical;
use maze_tools::cell::*;

use super::*;

/// A constant used as multiplier for individual colour values to get an
/// intensity
const D: f32 = 1.0 / 255.0 / 3.0;

/// A masking image.
pub struct MaskInitializer<R>
where
    R: initialize::Randomizer + Sized,
{
    /// The mask image.
    pub image: image::RgbImage,

    /// The intensity threshold
    pub threshold: f32,

    _marker: ::std::marker::PhantomData<R>,
}

impl<R> FromStr for MaskInitializer<R>
where
    R: initialize::Randomizer + Sized,
{
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
                    _marker: ::std::marker::PhantomData,
                })
            } else {
                Err(format!("invalid threshold: {}", part1))
            }
        } else {
            Err(format!("invalid mask: {}", s))
        }
    }
}

impl<R> Initializer<R> for MaskInitializer<R>
where
    R: initialize::Randomizer + Sized,
{
    /// Applies the initialise action.
    ///
    /// This action will use the intensity of pixels to determine whether
    /// rooms should be part of the maze.
    ///
    /// # Arguments
    /// *  `maze` - The maze to initialise.
    /// *  `rng` - A random number generator.
    /// *  `methods` - The initialisers to use to generate the maze.
    fn initialize(&self, maze: Maze, rng: &mut R, methods: Methods<R>) -> Maze {
        let physical::ViewBox { width, height, .. } = maze.viewbox();
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
            .split_by(&maze.shape(), maze.width(), maze.height())
            .map(|&v| v > self.threshold);

        methods.initialize(maze, rng, |pos| data[pos])
    }
}

#[derive(Clone, Copy, Default)]
struct Intermediate(f32);

impl<'a, P> From<&'a P> for Intermediate
where
    P: image::Pixel<Subpixel = u8>,
{
    fn from(source: &'a P) -> Self {
        Intermediate(source.channels().iter().map(|&b| f32::from(b)).sum())
    }
}

impl ops::Add<Intermediate> for Intermediate {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Intermediate(self.0 + other.0)
    }
}

impl ops::Div<usize> for Intermediate {
    type Output = f32;

    fn div(self, divisor: usize) -> Self::Output {
        D * self.0 / divisor as f32
    }
}

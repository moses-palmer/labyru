use std::collections::hash_map::HashMap;

use maze;

use maze::{matrix, physical};

#[macro_use]
mod macros;
pub mod default;

/// The width of a character bitmap.
const WIDTH: usize = 8;

/// The height of a character bitmap.
const HEIGHT: usize = 8;

/// The value `1.0f32` in a convenient representation for the alphabet macro.
const X: f32 = 1.0;

/// The value `0.0f32` in a convenient representation for the alphabet macro.
const O: f32 = 0.0;

/// A character bitmap.
pub struct Character(pub(self) [[f32; WIDTH]; HEIGHT]);

impl Character {
    /// Retrieves an interpolated bit from the bitmap.
    ///
    /// Positions outside of the bitmap are considered to be `0.0f32`.
    ///
    /// The bit at `(0, 0)` will have the greatest impact at the physical
    /// position `(0.5, 0.5)`.
    ///
    /// # Arguments
    /// *  `pos` - The position.
    pub fn interpolated(&self, pos: physical::Pos) -> f32 {
        // Since values are centered around (0.5, 0.5), we do not need to
        // interpolate values outside of the range
        if pos.x < 0.0
            || pos.y < 0.0
            || pos.x > WIDTH as f32
            || pos.y > HEIGHT as f32
        {
            0.0
        } else {
            let (col, dx) = matrix::partition(pos.x - 0.5);
            let (row, dy) = matrix::partition(pos.y - 0.5);

            let tl = self.get(matrix::Pos { col, row });
            let tr = self.get(matrix::Pos { col: col + 1, row });
            let t = tl * (1.0 - dx) + tr * dx;

            let bl = self.get(matrix::Pos { col, row: row + 1 });
            let br = self.get(matrix::Pos {
                col: col + 1,
                row: row + 1,
            });
            let b = bl * (1.0 - dx) + br * dx;

            t * (1.0 - dy) + b * dy
        }
    }

    /// Reads a specific bit.
    ///
    /// If the position is outside of the bitmap, `0.0f32` is returned.
    ///
    /// # Arguments
    /// *  `pos` - The position to read.
    fn get(&self, pos: matrix::Pos) -> f32 {
        if pos.col >= 0
            && pos.row >= 0
            && pos.col < WIDTH as isize
            && pos.row < HEIGHT as isize
        {
            self.0[pos.row as usize][pos.col as usize]
        } else {
            0.0
        }
    }
}

/// The bitmaps of an alphabet.
pub struct Alphabet {
    /// The default character used when a string contains unknown characters.
    pub(self) default: Character,

    /// A mapping from character to bitmap.
    pub(self) map: HashMap<char, Character>,
}

impl Alphabet {
    /// Generates an iterator over the pixels of a string rendered by this
    /// alphabet.
    ///
    /// # Arguments
    /// *  `text` - The text to render.
    /// *  `columns` - The number of columns. This determines the horisontal
    ///    size of the image. When reached, a line break will be added.
    /// *  `resolution` - The number of samples to generate horisontally.
    pub fn render<'a, 'b>(
        &'a self,
        text: &'b str,
        columns: usize,
        horizontal_resolution: usize,
    ) -> AlphabetRenderer<'a> {
        let rows = (text.len() as f32 / columns as f32).ceil() as usize;
        let text = text.chars().collect();
        let resolution = horizontal_resolution / columns;
        let current = 0;
        let limit = columns * rows * resolution * resolution;
        AlphabetRenderer {
            alphabet: self,
            text,
            columns,
            resolution,
            current,
            limit,
        }
    }
}

/// An iterator over bit samples for a rendered text.
pub struct AlphabetRenderer<'a> {
    /// The alphabet to use.
    alphabet: &'a Alphabet,

    /// The characters of the text.
    text: Vec<char>,

    /// The number of columns.
    columns: usize,

    /// The number of samples per character in each direction.
    resolution: usize,

    /// The current index.
    current: usize,

    /// The maximum number of samples.
    limit: usize,
}

impl<'a> AlphabetRenderer<'a> {
    /// Returns the current position.
    ///
    /// The position is represented as the tuple
    /// `(column * resolution, row * resolution)`.
    fn position(&self) -> (usize, usize) {
        let x = self.current % (self.columns * self.resolution);
        let y = self.current / (self.columns * self.resolution);
        (x, y)
    }
}

impl<'a> Iterator for AlphabetRenderer<'a> {
    type Item = (physical::Pos, f32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.limit {
            // Get the position
            let (ix, iy) = AlphabetRenderer::position(self);
            self.current += 1;

            // Calculate the character index
            let col = ix / self.resolution;
            let row = iy / self.resolution;
            let i = row * self.columns + col;

            // Calculate the physical position
            let x = ix as f32 / self.resolution as f32;
            let y = iy as f32 / self.resolution as f32;

            // Calculate the relative position within the character cell
            let rx = WIDTH as f32 * (ix - col * self.resolution) as f32
                / self.resolution as f32;
            let ry = HEIGHT as f32 * (iy - row * self.resolution) as f32
                / self.resolution as f32;

            Some((
                physical::Pos { x, y },
                self.text
                    .get(i)
                    .map(|&c| self.alphabet.get(c))
                    .map(|c| c.interpolated(physical::Pos { x: rx, y: ry }))
                    .unwrap_or(0.0),
            ))
        } else {
            None
        }
    }
}

impl Alphabet {
    /// Retrieves the bitmap for a character, or the default one if none exists.
    ///
    /// # Arguments
    /// *  `character` - The character for which to retrieve a bitmap.
    fn get(&self, character: char) -> &Character {
        self.map.get(&character).unwrap_or(&self.default)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn character_interpolated() {
        let character = Character(character! {
            O O O O X X X X
            O O O O X X X X
            O O O O X X X X
            O O O O X X X X
            X X X X X X X X
            X X X X X X X X
            X X X X X X X X
            X X X X X X X X
        });
        assert_eq!(
            character.interpolated(physical::Pos { x: -0.5, y: -0.5 }),
            0.0
        );
        assert_eq!(
            character.interpolated(physical::Pos { x: -0.5, y: 5.5 }),
            0.0
        );
        assert_eq!(
            character.interpolated(physical::Pos { x: -0.5, y: 9.5 }),
            0.0
        );
        assert_eq!(
            character.interpolated(physical::Pos { x: 8.5, y: -0.5 }),
            0.0
        );
        assert_eq!(
            character.interpolated(physical::Pos { x: 8.5, y: 5.5 }),
            0.0
        );
        assert_eq!(
            character.interpolated(physical::Pos { x: 8.5, y: 9.5 }),
            0.0
        );
        assert_eq!(
            character.interpolated(physical::Pos { x: 4.5, y: -0.5 }),
            0.0
        );
        assert_eq!(
            character.interpolated(physical::Pos { x: 4.5, y: 9.5 }),
            0.0
        );
        assert_eq!(
            character.interpolated(physical::Pos { x: 6.5, y: 2.5 }),
            1.0
        );
        assert_eq!(
            character.interpolated(physical::Pos { x: 4.0, y: 2.5 }),
            0.5
        );
        assert_eq!(
            character.interpolated(physical::Pos { x: 2.5, y: 4.0 }),
            0.5
        );
    }
}

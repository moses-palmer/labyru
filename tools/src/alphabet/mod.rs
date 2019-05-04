use std::collections::hash_map::HashMap;

use maze;

use maze::{matrix, physical};

#[macro_use]
mod macros;

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

            (t * (1.0 - dy) + b * dy)
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

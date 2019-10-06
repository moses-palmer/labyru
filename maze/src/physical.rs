use serde::{Deserialize, Serialize};

/// A physical position.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Pos {
    /// The X coordinate.
    pub x: f32,

    /// The Y coordinate.
    pub y: f32,
}

impl Pos {
    /// Returns the distance squared between two points.
    ///
    /// # Arguments
    /// *  `other` - The other point.
    pub fn distance_squared(self, other: Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }
}

impl<T> From<(T, T)> for Pos
where
    T: Into<f32>,
{
    fn from((x, y): (T, T)) -> Self {
        Pos {
            x: x.into(),
            y: y.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pos_into() {
        let expected = Pos { x: 1.0, y: 2.0 };
        let actual: Pos = (1.0, 2.0).into();
        assert_eq!(expected, actual);
    }

    #[test]
    fn distance_squared() {
        assert_eq!(
            4.0,
            Pos { x: 0.0, y: 0.0 }.distance_squared((2.0, 0.0).into()),
        );
        assert_eq!(
            4.0,
            Pos { x: 0.0, y: 0.0 }.distance_squared((0.0, 2.0).into()),
        );
    }
}

/// A physical position.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pos {
    /// The X coordinate.
    pub x: f32,

    /// The Y coordinate.
    pub y: f32,
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
}

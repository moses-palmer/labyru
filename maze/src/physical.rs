//! # Aspects of the maze as laid out in a physical landscape
//!
//! When physically laying out the maze, rooms and edges have certain
//! attributes. These are collected in this module.
use std::ops;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::wall::Angle;

/// A physical position.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Pos {
    /// The X coordinate.
    pub x: f32,

    /// The Y coordinate.
    pub y: f32,
}

impl Pos {
    /// A scalar value signifying distance from _(0, 0)_.
    ///
    /// # Example
    ///
    /// ```
    /// # use maze::physical::*;
    ///
    /// let d = (0.1f32.cos(), 0.1f32.sin());
    /// let v = (
    ///     Pos { x:       0.0, y:       0.0 }.value(),
    ///     Pos { x: d.0 * 1.0, y: d.1 * 1.0 }.value(),
    ///     Pos { x: d.0 * 2.0, y: d.1 * 2.0 }.value(),
    ///     Pos { x: d.0 * 4.0, y: d.1 * 4.0 }.value(),
    /// );
    ///
    /// assert!(v.0 < v.1);
    /// assert!(v.1 < v.2);
    /// assert!(v.2 < v.3);
    /// ```
    pub fn value(self) -> f32 {
        self.x * self.x + self.y * self.y
    }
}

impl<T> From<(T, T)> for Pos
where
    T: Into<f32>,
{
    /// Converts the tuple _(x, y)_ to `Pos { x, y }`.
    ///
    /// # Example
    ///
    /// ```
    /// # use maze::physical::*;
    ///
    /// assert_eq!(
    ///     Pos::from((1.0f32,    2.0f32)),
    ///     Pos {   x: 1.0,    y: 2.0 },
    /// );
    /// assert_eq!(
    ///     Pos::from((1i16,      2i16)),
    ///     Pos {   x: 1.0,    y: 2.0 },
    /// );
    /// ```
    fn from((x, y): (T, T)) -> Self {
        Pos {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl ops::Add for Pos {
    type Output = Self;

    /// Adds the axis values of two positions.
    ///
    /// # Example
    ///
    /// ```
    /// # use maze::physical::*;
    ///
    /// assert_eq!(
    ///     Pos { x: 1.0, y: 2.0 } + Pos { x: 3.0, y: 4.0 },
    ///     Pos { x: 4.0, y: 6.0 },
    /// );
    /// ```
    ///
    /// # Arguments
    /// *  `other` - The other position to add.
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub for Pos {
    type Output = Self;

    /// Subtracts the axis values of another position from the axis values of
    /// this one.
    ///
    /// # Example
    ///
    /// ```
    /// # use maze::physical::*;
    ///
    /// assert_eq!(
    ///     Pos { x: 4.0, y: 6.0 } - Pos { x: 3.0, y: 4.0 },
    ///     Pos { x: 1.0, y: 2.0 },
    /// );
    /// ```
    ///
    /// # Arguments
    /// *  `other` - The other position to add.
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::Add<Angle> for Pos {
    type Output = Self;

    /// Adds the delta values of an angle to this position.
    ///
    /// # Example
    ///
    /// ```
    /// # use maze::physical::*;
    /// # use maze::wall::*;
    ///
    /// assert_eq!(
    ///     Pos { x: 1.0, y: 2.0 } + Angle { a: 0.0, dx: 1.0, dy: 0.0 },
    ///     Pos { x: 2.0, y: 2.0 },
    /// );
    /// ```
    ///
    /// # Arguments
    /// *  `other` - The other position to add.
    fn add(self, other: Angle) -> Self {
        Self {
            x: self.x + other.dx,
            y: self.y + other.dy,
        }
    }
}

/// A view box described by one corner and the width and height of the sides.
///
/// The remaining corners are retrieved by adding the width and height the the
/// corner coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewBox {
    /// A corner.
    ///
    /// The coordinates of the remaining corners can be calculated by adding
    /// `width` and `height` to this value.
    pub corner: Pos,

    /// The width of the view box.
    pub width: f32,

    /// The height of the view box.
    pub height: f32,
}

impl ViewBox {
    /// Creates a view box centered around a point.
    ///
    /// # Arguments
    /// *  `pos` - The centre.
    /// *  `width` - The width of the view box.
    /// *  `height` - The height of the view box.
    pub fn centered_at(pos: Pos, width: f32, height: f32) -> Self {
        Self {
            corner: Pos {
                x: pos.x - 0.5 * width,
                y: pos.y - 0.5 * height,
            },
            width,
            height,
        }
    }

    /// Flattens this view box to the tuple `(x, y, width, height)`.
    ///
    /// # Example
    ///
    /// ```
    /// # use maze::physical::*;
    ///
    /// assert_eq!(
    ///     ViewBox {
    ///         corner: Pos {
    ///             x: 1.0,
    ///             y: 2.0,
    ///         },
    ///         width: 3.0,
    ///         height: 4.0,
    ///     }.tuple(),
    ///     (1.0, 2.0, 3.0, 4.0),
    /// );
    /// ```
    pub fn tuple(self) -> (f32, f32, f32, f32) {
        (self.corner.x, self.corner.y, self.width, self.height)
    }

    /// Expands this view box with `d` units.
    ///
    /// The centre is maintained, but every side will be `d` units further from
    /// it.
    ///
    /// If `d` is a negative value, the view box will be contracted, which may
    /// lead to a view box with negative dimensions.
    ///
    /// # Example
    ///
    /// ```
    /// # use maze::physical::*;
    ///
    /// assert_eq!(
    ///     ViewBox::centered_at(Pos { x: 1.0, y: 1.0 }, 2.0, 2.0)
    ///         .expand(1.0),
    ///     ViewBox {
    ///         corner: Pos {
    ///             x: -1.0,
    ///             y: -1.0,
    ///         },
    ///         width: 4.0,
    ///         height: 4.0,
    ///     },
    /// );
    /// ```
    ///
    /// # Arguments
    /// *  `d` - The number of units to expand.
    pub fn expand(self, d: f32) -> Self {
        Self {
            corner: Pos {
                x: self.corner.x - d,
                y: self.corner.y - d,
            },
            width: self.width + 2.0 * d,
            height: self.height + 2.0 * d,
        }
    }

    /// The centre of this view box.
    ///
    /// # Example
    ///
    /// ```
    /// # use maze::physical::*;
    ///
    /// let viewbox = ViewBox {
    ///     corner: Pos { x: 0.0, y: 0.0 },
    ///     width: 2.0,
    ///     height: 2.0,
    /// };
    ///
    /// assert_eq!(
    ///     viewbox.center(),
    ///     Pos { x: 1.0, y: 1.0 },
    /// );
    /// ```
    pub fn center(self) -> Pos {
        Pos {
            x: self.corner.x + 0.5 * self.width,
            y: self.corner.y + 0.5 * self.height,
        }
    }

    /// Whether a point is inside this view box.
    ///
    /// Points along the edge of the view box are also considered to be inside.
    ///
    /// # Example
    ///
    /// ```
    /// # use maze::physical::*;
    ///
    /// let viewbox = ViewBox {
    ///     corner: Pos { x: 0.0, y: 0.0 },
    ///     width: 1.0,
    ///     height: 1.0,
    /// };
    /// assert!(viewbox.contains(Pos { x: 0.0, y: 0.0 }));
    /// assert!(viewbox.contains(Pos { x: 0.5, y: 0.5 }));
    /// assert!(viewbox.contains(Pos { x: 1.0, y: 1.0 }));
    /// assert!(!viewbox.contains(Pos { x: 2.0, y: 2.0 }));
    /// ```
    ///
    /// # Arguments
    /// *  `pos` - The position to check.
    pub fn contains(self, pos: Pos) -> bool {
        pos.x >= self.corner.x
            && pos.y >= self.corner.y
            && pos.x <= self.corner.x + self.width
            && pos.y <= self.corner.x + self.height
    }
}

impl ops::Mul<ViewBox> for f32 {
    type Output = ViewBox;

    /// Scales every value in this view box by `rhs`.
    ///
    /// # Example
    ///
    /// ```
    /// # use maze::physical::*;
    ///
    /// assert_eq!(
    ///     2.0 * ViewBox {
    ///         corner: Pos { x: 1.0, y: 1.0 },
    ///         width: 2.0,
    ///         height: 2.0,
    ///     },
    ///     ViewBox {
    ///         corner: Pos { x: 2.0, y: 2.0 },
    ///         width: 4.0,
    ///         height: 4.0,
    ///     },
    /// );
    /// ```
    ///
    /// # Arguments
    /// *  `rhs` - The view box to scale.
    fn mul(self, rhs: ViewBox) -> Self::Output {
        Self::Output {
            corner: Pos {
                x: rhs.corner.x * self,
                y: rhs.corner.y * self,
            },
            width: rhs.width * self,
            height: rhs.height * self,
        }
    }
}

impl ops::Mul<f32> for ViewBox {
    type Output = Self;

    /// Scales every value in a view box by this value.
    fn mul(self, rhs: f32) -> Self::Output {
        rhs * self
    }
}

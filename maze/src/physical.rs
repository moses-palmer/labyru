//! # Aspects of the maze as laid out in a physical landscape
//!
//! When physically laying out the maze, rooms and edges have certain
//! attributes. These are collected in this module.
use std::ops;

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
    /// A scalar value signifying distance from _(0, 0)_.
    pub fn value(self) -> f32 {
        self.x * self.x + self.y * self.y
    }
}

impl<T> From<(T, T)> for Pos
where
    T: Into<f32>,
{
    /// Converts the tuple _(x, y)_ to `Pos { x, y }`.
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
    /// # Arguments
    /// *  `other` - The other position to add.
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
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
    pub fn center(self) -> Pos {
        Pos {
            x: self.corner.x + 0.5 * self.width,
            y: self.corner.y + 0.5 * self.height,
        }
    }

    /// Determines whether a point is inside this view box.
    ///
    /// Points along the edge of the view box are also considered to be inside.
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

    fn mul(self, rhs: f32) -> Self::Output {
        rhs * self
    }
}

use std;

use serde::{Deserialize, Serialize};

/// The maximum nomalised value of a radian.
const RADIAN_BOUND: f32 = 2.0 * std::f32::consts::PI;

/// A wall index.
pub type Index = usize;

/// A bit mask for a wall.
pub type Mask = u32;

/// An offset from a wall to its corner neighbours.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Offset {
    /// The horisontal offset.
    pub dx: isize,

    /// The vertical offset.
    pub dy: isize,

    /// The neighbour index.
    pub wall: Index,
}

/// An angle in a span.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Angle {
    /// The angle.
    pub a: f32,

    /// cos(a).
    pub dx: f32,

    /// sin(a).
    pub dy: f32,
}

/// A wall.
///
/// Walls have an index, which is used by [Room](../room/struct.Room.html) to
/// generate bit masks, and a direction, which indicates the position of the
/// room on the other side of a wall, relative to the room to which the wall
/// belongs.
#[derive(Clone, PartialOrd, Serialize)]
pub struct Wall {
    /// The name of this wall.
    pub name: &'static str,

    /// The index of this wall, used to generate the bit mask.
    pub index: Index,

    /// Offsets to other walls in the first corner of this wall.
    pub corner_wall_offsets: &'static [Offset],

    /// The horizontal and vertical offset of the room on the other side of this
    /// wall.
    pub dir: (isize, isize),

    /// The span, in radians, of the wall.
    ///
    /// The first value is the start of the span, and the second value the end.
    /// The second value will always be greater, even if the span wraps around
    /// _2𝜋_.
    pub span: (Angle, Angle),
}

impl Wall {
    /// The bit mask for this wall.
    pub fn mask(&self) -> Mask {
        1 << self.index
    }

    /// Normalises an angle to be in the bound _[0, 2𝜋)_.
    ///
    /// # Arguments
    /// *  `angle` - The angle to normalise.
    pub fn normalized_angle(angle: f32) -> f32 {
        if angle < RADIAN_BOUND && angle >= 0.0 {
            angle
        } else {
            let t = angle % RADIAN_BOUND;
            if t >= 0.0 {
                t
            } else {
                t + RADIAN_BOUND
            }
        }
    }

    /// Whether an angle is in the span of this wall.
    ///
    /// The angle will be normalised.
    ///
    /// # Arguments
    /// *  `angle` - The angle in radians.
    pub fn in_span(&self, angle: f32) -> bool {
        let normalized = Wall::normalized_angle(angle);

        if (self.span.0.a <= normalized) && (normalized < self.span.1.a) {
            true
        } else {
            let overflowed = normalized + RADIAN_BOUND;
            (self.span.0.a <= overflowed) && (overflowed < self.span.1.a)
        }
    }
}

impl PartialEq for Wall {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.dir == other.dir
    }
}

impl Eq for Wall {}

impl std::hash::Hash for Wall {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.index.hash(state);
        self.dir.hash(state);
    }
}

impl std::fmt::Debug for Wall {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.write_str(self.name)
    }
}

impl Ord for Wall {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}

use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::shape::Shape;

/// The maximum nomalised value of a radian.
const RADIAN_BOUND: f32 = 2.0 * std::f32::consts::PI;

/// A wall index.
pub type Index = usize;

/// A bit mask for a wall.
pub type Mask = u32;

/// An offset from a wall to its corner neighbours.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Offset {
    /// The horisontal offset.
    pub dx: isize,

    /// The vertical offset.
    pub dy: isize,

    /// The neighbour index.
    pub wall: &'static Wall,
}

/// An angle in a span.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
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
#[derive(Clone)]
pub struct Wall {
    /// The name of this wall.
    pub name: &'static str,

    /// The shape to which this wall belongs.
    pub shape: Shape,

    /// The ordinal of this wall.
    ///
    /// The ordinals will be in the range _[0, n)_, where _n_ is the number of
    /// walls for the shape. When listing the walls of a room, the sequence
    /// number of a wall will be equal to this number.
    pub ordinal: usize,

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
    /// The second value will be smaller if the span wraps around _2𝜋_.
    pub span: (Angle, Angle),

    /// The previous wall, clock-wise.
    pub previous: &'static Wall,

    /// The next wall, clock-wise.
    pub next: &'static Wall,
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

        if self.span.0.a < self.span.1.a {
            (self.span.0.a <= normalized) && (normalized < self.span.1.a)
        } else {
            (self.span.0.a <= normalized) || (normalized < self.span.1.a)
        }
    }
}

impl PartialEq for Wall {
    fn eq(&self, other: &Self) -> bool {
        self.shape == other.shape
            && self.index == other.index
            && self.dir == other.dir
    }
}

impl Eq for Wall {}

impl std::hash::Hash for Wall {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.shape.hash(state);
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

impl PartialOrd for Wall {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl<'de> Deserialize<'de> for &'static Wall {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wall_name = String::deserialize(deserializer)?;
        crate::shape::hex::walls::ALL
            .iter()
            .chain(crate::shape::quad::walls::ALL.iter())
            .chain(crate::shape::tri::walls::ALL.iter())
            .find(|wall| wall.name == wall_name)
            .copied()
            .ok_or_else(|| D::Error::custom("expected a wall name"))
    }
}

impl Serialize for Wall {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.name)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use maze_test::maze_test;

    use super::*;
    use crate::*;
    use test_utils::*;

    #[maze_test]
    fn unique(maze: TestMaze) {
        let walls = maze.walls(matrix_pos(0, 1));
        assert_eq!(
            walls
                .iter()
                .cloned()
                .collect::<HashSet<&wall::Wall>>()
                .len(),
            walls.len()
        );
    }

    #[maze_test]
    fn ordinal(maze: TestMaze) {
        for pos in maze.positions() {
            for (i, wall) in maze.walls(pos).iter().enumerate() {
                assert_eq!(i, wall.ordinal, "invalid ordinal for {:?}", wall);
            }
        }
    }

    #[maze_test]
    fn span(maze: TestMaze) {
        fn assert_span(wall: &'static Wall, angle: f32) {
            assert!(
                wall.in_span(angle),
                "{} was not in span ({} - {}) for {:?}",
                angle,
                wall.span.0.a,
                wall.span.1.a,
                wall,
            );
        }

        fn assert_not_span(wall: &'static Wall, angle: f32) {
            assert!(
                !wall.in_span(angle),
                "{} was in span ({} - {}) for {:?}",
                angle,
                wall.span.0.a,
                wall.span.1.a,
                wall,
            );
        }

        for pos in maze.positions() {
            for wall in maze.walls(pos) {
                let d = 16.0 * std::f32::EPSILON;
                assert_span(wall, wall.span.0.a + d);
                assert_not_span(wall, wall.span.0.a - d);
                assert_span(wall.previous, wall.span.0.a - d);
                assert_span(wall, wall.span.1.a - d);
                assert_not_span(wall, wall.span.1.a + d);
                assert_span(wall.next, wall.span.1.a + d);

                assert!(
                    nearly_equal(wall.span.0.a.cos(), wall.span.0.dx),
                    "{} span 0 dx invalid ({} != {})",
                    wall.name,
                    wall.span.0.a.cos(),
                    wall.span.0.dx,
                );
                assert!(
                    nearly_equal(wall.span.0.a.sin(), wall.span.0.dy),
                    "{} span 0 dy invalid ({} != {})",
                    wall.name,
                    wall.span.0.a.sin(),
                    wall.span.0.dy,
                );
                assert!(
                    nearly_equal(wall.span.1.a.cos(), wall.span.1.dx),
                    "{} span 1 dx invalid ({} != {})",
                    wall.name,
                    wall.span.1.a.cos(),
                    wall.span.1.dx,
                );
                assert!(
                    nearly_equal(wall.span.1.a.sin(), wall.span.1.dy),
                    "{} span 1 dy invalid ({} != {})",
                    wall.name,
                    wall.span.1.a.sin(),
                    wall.span.1.dy,
                );
            }
        }
    }

    #[maze_test]
    fn order(maze: TestMaze) {
        for pos in maze.positions() {
            let walls = maze.walls(pos);
            for wall in walls {
                let d = 16.0 * std::f32::EPSILON;
                assert!(
                    wall.in_span(wall.previous.span.1.a + d),
                    "invalid wall order {:?}: {:?} <=> {:?}",
                    walls,
                    wall.previous,
                    wall,
                );
                assert!(
                    wall.in_span(wall.next.span.0.a - d),
                    "invalid wall order {:?}: {:?} <=> {:?}",
                    walls,
                    wall,
                    wall.next,
                );
            }
        }
    }

    #[maze_test]
    fn wall_serialization(maze: TestMaze) {
        for wall in maze.all_walls() {
            let serialized = serde_json::to_string(wall).unwrap();
            let deserialized: &'static Wall =
                serde_json::from_str(&serialized).unwrap();
            assert_eq!(*wall, deserialized);
        }
    }

    #[maze_test]
    fn in_span(maze: TestMaze) {
        let mut failures = Vec::new();
        let count = 100_000;

        // Test for two different rooms to ensure we cover all room types
        for col in 0..=1 {
            failures.extend(
                (0..=count)
                    .map(|t| {
                        2.0 * (RADIAN_BOUND * (t as f32 / count as f32)
                            - std::f32::consts::PI)
                    })
                    .filter(|&a| {
                        maze.walls(matrix::Pos { col, row: 0 })
                            .iter()
                            .filter(|wall| wall.in_span(a))
                            .next()
                            .is_none()
                    }),
            );
        }

        assert_eq!(
            Vec::<f32>::new(),
            failures,
            "not all angles were in the span of a wall ({}% failed)",
            100.0 * (failures.len() as f32 / (2.0 * count as f32)),
        );
    }
}

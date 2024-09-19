use serde::{Deserialize, Serialize};

use crate::matrix;
use crate::Maze;

/// A list of instructions.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Instructions(Vec<Instruction>);

/// Instructions for the spelunker.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Instruction {
    /// Move forward.
    Forward,

    /// Turn left.
    Left,

    /// Turn right.
    Right,

    /// Fork and let one spelunker start from the beginning of the instruction
    /// set after having turned left.
    ForkLeft,

    /// Fork and let one spelunker start from the beginning of the instruction
    /// set after having turned right.
    ForkRight,
}

/// Initialises a maze by clearing all inner walls.
///
/// This method will ignore rooms for which `filter` returns `false`.
///
/// # Arguments
/// *  `maze``- The maze to initialise.
/// *  `rng` - Not used.
/// *  `candidates` - A predicate filtering rooms to consider.
/// *  `instructions` - The spelunker instructions.
pub(crate) fn initialize<R, T>(
    mut maze: Maze<T>,
    rng: &mut R,
    mut candidates: matrix::Matrix<bool>,
    instructions: &Instructions,
) -> Maze<T>
where
    R: super::Randomizer + Sized,
    T: Clone,
{
    let mask = candidates.clone();

    // The list of spelunker origins
    let mut origins = super::random_room(rng, &candidates)
        .and_then(|pos| super::random_wall(rng, &candidates, pos, &maze))
        .into_iter()
        .collect::<Vec<_>>();

    loop {
        // Find the next wall position
        let mut wall_pos = if let Some(wall_pos) = origins.pop() {
            // Continue a previous fork
            wall_pos
        } else {
            if let Some(room_pos) = super::random_room(rng, &candidates) {
                candidates[room_pos] = false;
                if let Some(wall_pos) =
                    super::random_wall(rng, &candidates, room_pos, &maze)
                {
                    // We have selected a new origin
                    wall_pos
                } else {
                    // The rooms has no candidate walls
                    continue;
                }
            } else {
                // All candidate rooms have been visited
                break;
            }
        };

        // Execute the instructions
        for i in instructions.0.iter().cycle() {
            use Instruction::*;
            match i {
                Forward => {
                    candidates[wall_pos.0] = false;

                    let back = maze.back(wall_pos);
                    if candidates.get(back.0).is_some_and(|&b| b)
                        && !maze.rooms[back.0].visited
                    {
                        maze.open(wall_pos);
                        wall_pos = maze
                            .opposite(back)
                            .map(|wall| (back.0, wall))
                            .unwrap();
                        candidates[wall_pos.0] = false;
                    } else {
                        break;
                    }
                }
                Left => wall_pos = (wall_pos.0, wall_pos.1.previous),
                Right => wall_pos = (wall_pos.0, wall_pos.1.next),
                ForkLeft => origins.push((wall_pos.0, wall_pos.1.previous)),
                ForkRight => origins.push((wall_pos.0, wall_pos.1.next)),
            }
        }
    }

    println!("CONNECTING");
    super::connect_all(&mut maze, rng, |pos| mask.get(pos).is_some_and(|&b| b));

    maze
}

/// The prefix for the string version of this method.
const PREFIX: &str = "spelunker(";

/// The suffix for the string version of this method.
const SUFFIX: &str = ")";

pub(crate) fn parse_method(s: &str) -> Result<super::Method, ()> {
    if s.starts_with(PREFIX) && s.ends_with(SUFFIX) {
        Ok(super::Method::Spelunker {
            instructions: s[PREFIX.len()..s.len() - SUFFIX.len()].parse()?,
        })
    } else {
        Err(())
    }
}

pub(crate) fn display_method(
    f: &mut std::fmt::Formatter,
    instructions: &Instructions,
) -> std::fmt::Result {
    write!(f, "{}{}{}", PREFIX, instructions, SUFFIX)
}

impl From<Vec<Instruction>> for Instructions {
    fn from(source: Vec<Instruction>) -> Self {
        Self(source)
    }
}

impl std::fmt::Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in &self.0 {
            write!(f, "{}", i.char())?;
        }
        Ok(())
    }
}

impl std::str::FromStr for Instructions {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars()
                .map(|c| Instruction::try_from(c))
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

impl Instruction {
    /// The character used to represent this instruction.
    pub const fn char(self) -> char {
        use Instruction::*;
        match self {
            Forward => '|',
            Left => '<',
            Right => '>',
            ForkLeft => '}',
            ForkRight => '{',
        }
    }
}

impl TryFrom<char> for Instruction {
    type Error = ();

    fn try_from(source: char) -> Result<Self, Self::Error> {
        use Instruction::*;
        match source {
            c if c == Forward.char() => Ok(Forward),
            c if c == Left.char() => Ok(Left),
            c if c == Right.char() => Ok(Right),
            c if c == ForkLeft.char() => Ok(ForkLeft),
            c if c == ForkRight.char() => Ok(ForkRight),
            _ => Err(()),
        }
    }
}

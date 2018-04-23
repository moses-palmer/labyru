use std;

use super::Shape;
use matrix;
use physical;
use room;
use wall;
use WallPos;

/// A span step angle
const D: f32 = std::f32::consts::PI / 4.0;

/// The scale factor when converting maze coordinates to physical coordinates
const MULTIPLICATOR: f32 = 2.0 / std::f32::consts::SQRT_2;

define_walls! {
    UP = {
        corner_wall_offsets: &[
            ((0, -1), WallIndex::LEFT as usize),
            ((-1, -1), WallIndex::DOWN as usize),
            ((-1, 0), WallIndex::RIGHT as usize),
        ],
        dir: (0, -1),
        span: (5.0 * D, 7.0 * D),
    },
    LEFT = {
        corner_wall_offsets: &[
            ((-1, 0), WallIndex::DOWN as usize),
            ((-1, 1), WallIndex::RIGHT as usize),
            ((0, 1), WallIndex::UP as usize),
        ],
        dir: (-1, 0),
        span: (3.0 * D, 5.0 * D),
    },
    DOWN = {
        corner_wall_offsets: &[
            ((0, 1), WallIndex::RIGHT as usize),
            ((1, 1), WallIndex::UP as usize),
            ((1, 0), WallIndex::LEFT as usize),
        ],
        dir: (0, 1),
        span: (D, 3.0 * D),
    },
    RIGHT = {
        corner_wall_offsets: &[
            ((1, 0), WallIndex::UP as usize),
            ((1, -1), WallIndex::LEFT as usize),
            ((0, -1), WallIndex::DOWN as usize),
        ],
        dir: (1, 0),
        span: (7.0 * D, 9.0 * D),
    }
}

/// The index of the back wall.
macro_rules! back_index {
    ($wall:expr) => {
        $wall ^ 0b0010
    };
}

/// The walls for a matrix position.
macro_rules! walls {
    ($pos:expr) => {
        &ALL
    };
}

/// The walls
static ALL: &[&'static wall::Wall] = &[
    &walls::LEFT,
    &walls::UP,
    &walls::RIGHT,
    &walls::DOWN,
];

define_base!();

impl Shape for Maze {
    implement_base_shape!();

    fn opposite(&self, wall_pos: WallPos) -> Option<&'static wall::Wall> {
        let (_, wall) = wall_pos;
        Some(
            &walls::ALL[(wall.index + walls::ALL.len() / 2) % walls::ALL.len()],
        )
    }
}

impl physical::Physical for Maze {
    fn center(&self, pos: matrix::Pos) -> physical::Pos {
        (
            (pos.0 as f32 + 0.5) * MULTIPLICATOR,
            (pos.1 as f32 + 0.5) * MULTIPLICATOR,
        )
    }

    fn room_at(&self, pos: physical::Pos) -> matrix::Pos {
        (
            (pos.0 / MULTIPLICATOR).floor() as isize,
            (pos.1 / MULTIPLICATOR).floor() as isize,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::*;
    use Walkable;
    use WallPos;

    #[test]
    fn back() {
        let maze = Maze::new(5, 5);

        assert_eq!(
            maze.back(((1, 1), &walls::LEFT)),
            ((0, 1), &walls::RIGHT)
        );
        assert_eq!(
            maze.back(((1, 1), &walls::UP)),
            ((1, 0), &walls::DOWN)
        );
        assert_eq!(
            maze.back(((1, 1), &walls::RIGHT)),
            ((2, 1), &walls::LEFT)
        );
        assert_eq!(
            maze.back(((1, 1), &walls::DOWN)),
            ((1, 2), &walls::UP)
        );
    }

    #[test]
    fn opposite() {
        let maze = Maze::new(5, 5);

        assert_eq!(
            maze.opposite(((1, 1), &walls::LEFT)).unwrap(),
            &walls::RIGHT
        );
        assert_eq!(
            maze.opposite(((1, 1), &walls::UP)).unwrap(),
            &walls::DOWN
        );
        assert_eq!(
            maze.opposite(((1, 1), &walls::RIGHT)).unwrap(),
            &walls::LEFT
        );
        assert_eq!(
            maze.opposite(((1, 1), &walls::DOWN)).unwrap(),
            &walls::UP
        );
    }

    #[test]
    fn corner_walls() {
        let maze = Maze::new(5, 5);

        assert_eq!(
            maze.corner_walls(((1, 1), &walls::UP)),
            vec![
                ((1, 1), &walls::UP),
                ((1, 0), &walls::LEFT),
                ((0, 0), &walls::DOWN),
                ((0, 1), &walls::RIGHT),
            ]
        );

        assert_eq!(
            maze.corner_walls(((1, 1), &walls::LEFT)),
            vec![
                ((1, 1), &walls::LEFT),
                ((0, 1), &walls::DOWN),
                ((0, 2), &walls::RIGHT),
                ((1, 2), &walls::UP),
            ]
        );

        assert_eq!(
            maze.corner_walls(((1, 1), &walls::DOWN)),
            vec![
                ((1, 1), &walls::DOWN),
                ((1, 2), &walls::RIGHT),
                ((2, 2), &walls::UP),
                ((2, 1), &walls::LEFT),
            ]
        );

        assert_eq!(
            maze.corner_walls(((1, 1), &walls::RIGHT)),
            vec![
                ((1, 1), &walls::RIGHT),
                ((2, 1), &walls::UP),
                ((2, 0), &walls::LEFT),
                ((1, 0), &walls::DOWN),
            ]
        );
    }

    #[test]
    fn follow_wall_single_room() {
        let maze = Maze::new(5, 5);

        assert_eq!(
            vec![
                ((0, 0), &walls::LEFT),
                ((0, 0), &walls::UP),
                ((0, 0), &walls::RIGHT),
                ((0, 0), &walls::DOWN),
            ],
            maze.follow_wall(((0, 0), &walls::LEFT))
                .map(|(from, _)| from)
                .collect::<Vec<WallPos>>()
        );
    }

    #[test]
    fn follow_wall() {
        let mut maze = Maze::new(5, 5);

        Navigator::new(&mut maze)
            .from((0, 0))
            .down(true)
            .right(true)
            .up(true);

        assert_eq!(
            vec![
                ((0, 0), &walls::LEFT),
                ((0, 0), &walls::UP),
                ((0, 0), &walls::RIGHT),
                ((1, 0), &walls::LEFT),
                ((1, 0), &walls::UP),
                ((1, 0), &walls::RIGHT),
                ((1, 1), &walls::RIGHT),
                ((1, 1), &walls::DOWN),
                ((0, 1), &walls::DOWN),
                ((0, 1), &walls::LEFT),
            ],
            maze.follow_wall(((0, 0), &walls::LEFT))
                .map(|(from, _)| from)
                .collect::<Vec<WallPos>>()
        );
    }
}

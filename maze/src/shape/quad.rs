use std;

use crate::matrix;
use crate::physical;
use crate::wall;

use crate::wall::{Index, Offset};
use crate::WallPos;

/// A span step angle
const D: f32 = std::f32::consts::PI / 4.0;

/// The scale factor when converting maze coordinates to physical coordinates
const MULTIPLICATOR: f32 = 2.0 / std::f32::consts::SQRT_2;

define_shape! {
    UP = {
        corner_wall_offsets: &[
            Offset { dx: 0, dy: -1, wall: WallIndex::LEFT as Index },
            Offset { dx: -1, dy: -1, wall: WallIndex::DOWN as Index },
            Offset { dx: -1, dy: 0, wall: WallIndex::RIGHT as Index },
        ],
        dir: (0, -1),
        span: (5.0 * D, 7.0 * D),
    },
    LEFT = {
        corner_wall_offsets: &[
            Offset { dx: -1, dy: 0, wall: WallIndex::DOWN as Index },
            Offset { dx: -1, dy: 1, wall: WallIndex::RIGHT as Index },
            Offset { dx: 0, dy: 1, wall: WallIndex::UP as Index },
        ],
        dir: (-1, 0),
        span: (3.0 * D, 5.0 * D),
    },
    DOWN = {
        corner_wall_offsets: &[
            Offset { dx: 0, dy: 1, wall: WallIndex::RIGHT as Index },
            Offset { dx: 1, dy: 1, wall: WallIndex::UP as Index },
            Offset { dx: 1, dy: 0, wall: WallIndex::LEFT as Index },
        ],
        dir: (0, 1),
        span: (D, 3.0 * D),
    },
    RIGHT = {
        corner_wall_offsets: &[
            Offset { dx: 1, dy: 0, wall: WallIndex::UP as Index },
            Offset { dx: 1, dy: -1, wall: WallIndex::LEFT as Index },
            Offset { dx: 0, dy: -1, wall: WallIndex::DOWN as Index },
        ],
        dir: (1, 0),
        span: (7.0 * D, 9.0 * D),
    }
}

/// The walls
static ALL: &[&wall::Wall] =
    &[&walls::LEFT, &walls::UP, &walls::RIGHT, &walls::DOWN];

pub fn minimal_dimensions(width: f32, height: f32) -> (usize, usize) {
    let height = (height.max(MULTIPLICATOR) / MULTIPLICATOR).ceil() as usize;

    let width = (width.max(MULTIPLICATOR) / MULTIPLICATOR).ceil() as usize;

    (width, height)
}

pub fn back_index(wall: usize) -> usize {
    wall ^ 0b0010
}

pub fn opposite(wall_pos: WallPos) -> Option<&'static wall::Wall> {
    let (_, wall) = wall_pos;
    Some(&walls::ALL[(wall.index + walls::ALL.len() / 2) % walls::ALL.len()])
}

pub fn walls(_pos: matrix::Pos) -> &'static [&'static wall::Wall] {
    &ALL
}

pub fn center(pos: matrix::Pos) -> physical::Pos {
    physical::Pos {
        x: (pos.col as f32 + 0.5) * MULTIPLICATOR,
        y: (pos.row as f32 + 0.5) * MULTIPLICATOR,
    }
}

pub fn room_at(pos: physical::Pos) -> matrix::Pos {
    matrix::Pos {
        col: (pos.x / MULTIPLICATOR).floor() as isize,
        row: (pos.y / MULTIPLICATOR).floor() as isize,
    }
}

#[cfg(test)]
mod tests {
    use maze_test::maze_test;

    use super::*;
    use crate::test_utils::*;
    use crate::Maze;
    use crate::WallPos;

    #[maze_test(quad)]
    fn back(maze: &mut Maze) {
        assert_eq!(
            maze.back((matrix_pos(1, 1), &walls::LEFT)),
            (matrix_pos(0, 1), &walls::RIGHT)
        );
        assert_eq!(
            maze.back((matrix_pos(1, 1), &walls::UP)),
            (matrix_pos(1, 0), &walls::DOWN)
        );
        assert_eq!(
            maze.back((matrix_pos(1, 1), &walls::RIGHT)),
            (matrix_pos(2, 1), &walls::LEFT)
        );
        assert_eq!(
            maze.back((matrix_pos(1, 1), &walls::DOWN)),
            (matrix_pos(1, 2), &walls::UP)
        );
    }

    #[maze_test(quad)]
    fn opposite(maze: &mut Maze) {
        assert_eq!(
            maze.opposite((matrix_pos(1, 1), &walls::LEFT)).unwrap(),
            &walls::RIGHT
        );
        assert_eq!(
            maze.opposite((matrix_pos(1, 1), &walls::UP)).unwrap(),
            &walls::DOWN
        );
        assert_eq!(
            maze.opposite((matrix_pos(1, 1), &walls::RIGHT)).unwrap(),
            &walls::LEFT
        );
        assert_eq!(
            maze.opposite((matrix_pos(1, 1), &walls::DOWN)).unwrap(),
            &walls::UP
        );
    }

    #[maze_test(quad)]
    fn corner_walls(maze: &mut Maze) {
        assert_eq!(
            maze.corner_walls((matrix_pos(1, 1), &walls::UP))
                .collect::<Vec<_>>(),
            vec![
                (matrix_pos(1, 1), &walls::UP),
                (matrix_pos(1, 0), &walls::LEFT),
                (matrix_pos(0, 0), &walls::DOWN),
                (matrix_pos(0, 1), &walls::RIGHT),
            ],
        );

        assert_eq!(
            maze.corner_walls((matrix_pos(1, 1), &walls::LEFT))
                .collect::<Vec<_>>(),
            vec![
                (matrix_pos(1, 1), &walls::LEFT),
                (matrix_pos(0, 1), &walls::DOWN),
                (matrix_pos(0, 2), &walls::RIGHT),
                (matrix_pos(1, 2), &walls::UP),
            ],
        );

        assert_eq!(
            maze.corner_walls((matrix_pos(1, 1), &walls::DOWN))
                .collect::<Vec<_>>(),
            vec![
                (matrix_pos(1, 1), &walls::DOWN),
                (matrix_pos(1, 2), &walls::RIGHT),
                (matrix_pos(2, 2), &walls::UP),
                (matrix_pos(2, 1), &walls::LEFT),
            ],
        );

        assert_eq!(
            maze.corner_walls((matrix_pos(1, 1), &walls::RIGHT))
                .collect::<Vec<_>>(),
            vec![
                (matrix_pos(1, 1), &walls::RIGHT),
                (matrix_pos(2, 1), &walls::UP),
                (matrix_pos(2, 0), &walls::LEFT),
                (matrix_pos(1, 0), &walls::DOWN),
            ],
        );
    }

    #[maze_test(quad)]
    fn follow_wall_single_room(maze: &mut Maze) {
        assert_eq!(
            vec![
                (matrix_pos(0, 0), &walls::LEFT),
                (matrix_pos(0, 0), &walls::UP),
                (matrix_pos(0, 0), &walls::RIGHT),
                (matrix_pos(0, 0), &walls::DOWN),
            ],
            maze.follow_wall((matrix_pos(0, 0), &walls::LEFT))
                .map(|(from, _)| from)
                .collect::<Vec<WallPos>>()
        );
    }

    #[maze_test(quad)]
    fn follow_wall(maze: &mut Maze) {
        Navigator::new(maze)
            .from(matrix_pos(0, 0))
            .down(true)
            .right(true)
            .up(true);

        assert_eq!(
            vec![
                (matrix_pos(0, 0), &walls::LEFT),
                (matrix_pos(0, 0), &walls::UP),
                (matrix_pos(0, 0), &walls::RIGHT),
                (matrix_pos(1, 0), &walls::LEFT),
                (matrix_pos(1, 0), &walls::UP),
                (matrix_pos(1, 0), &walls::RIGHT),
                (matrix_pos(1, 1), &walls::RIGHT),
                (matrix_pos(1, 1), &walls::DOWN),
                (matrix_pos(0, 1), &walls::DOWN),
                (matrix_pos(0, 1), &walls::LEFT),
            ],
            maze.follow_wall((matrix_pos(0, 0), &walls::LEFT))
                .map(|(from, _)| from)
                .collect::<Vec<WallPos>>()
        );
    }
}

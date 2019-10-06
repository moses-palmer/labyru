use std::f32::consts::PI;

use crate::WallPos;

use crate::matrix;
use crate::physical;
use crate::wall;

use crate::wall::{Angle, Index, Offset};

use super::{COS_30, SIN_30};

/// A span step angle
const D: f32 = PI / 6.0;

/// The distance between the centre of a room and the centre of a room on the
/// next row.
const HORIZONTAL_MULTIPLICATOR: f32 = COS_30;

/// The distance between the centre of a room and the centre of a room on the
/// next column.
const VERTICAL_MULTIPLICATOR: f32 = 2.0 - 1.0f32 / 2.0f32;

/// The vertical offset for the centre of rooms.
const OFFSET: f32 = 1.0f32 / 4.0f32;

define_shape! {
    LEFT0 = {
        corner_wall_offsets: &[
            Offset { dx: -1, dy: 0, wall: WallIndex::DOWN as Index },
            Offset { dx: -1, dy: 1, wall: WallIndex::RIGHT0 as Index },
            Offset { dx: 0, dy: 1, wall: WallIndex::RIGHT1 as Index },
            Offset { dx: 1, dy: 1, wall: WallIndex::UP as Index },
            Offset { dx: 1, dy: 0, wall: WallIndex::LEFT1 as Index },
        ],
        dir: (-1, 0),
        span: (
            Angle {
                a: 3.0 * D,
                dx: 0.0,
                dy: 1.0,
            },
            Angle {
                a: 7.0 * D,
                dx: -COS_30,
                dy: -SIN_30,
            },
        ),
    },
    RIGHT1 = {
        corner_wall_offsets: &[
            Offset { dx: 1, dy: 0, wall: WallIndex::UP as Index },
            Offset { dx: 1, dy: -1, wall: WallIndex::LEFT1 as Index },
            Offset { dx: 0, dy: -1, wall: WallIndex::LEFT0 as Index },
            Offset { dx: -1, dy: -1, wall: WallIndex::DOWN as Index },
            Offset { dx: -1, dy: 0, wall: WallIndex::RIGHT0 as Index },
        ],
        dir: (1, 0),
        span: (
            Angle {
                a: 9.0 * D,
                dx: 0.0,
                dy: -1.0,
            },
            Angle {
                a: 13.0 * D,
                dx: COS_30,
                dy: SIN_30,
            },
        ),
    },

    LEFT1 = {
        corner_wall_offsets: &[
            Offset { dx: -1, dy: 0, wall: WallIndex::LEFT0 as Index },
            Offset { dx: -2, dy: 0, wall: WallIndex::DOWN as Index },
            Offset { dx: -2, dy: 1, wall: WallIndex::RIGHT0 as Index },
            Offset { dx: -1, dy: 1, wall: WallIndex::RIGHT1 as Index },
            Offset { dx: 0, dy: 1, wall: WallIndex::UP as Index },
        ],
        dir: (-1, 0),
        span: (
            Angle {
                a: 5.0 * D,
                dx: -COS_30,
                dy: SIN_30,
            },
            Angle {
                a: 9.0 * D,
                dx: 0.0,
                dy: -1.0,
            },
        ),
    },
    RIGHT0 = {
        corner_wall_offsets: &[
            Offset { dx: 1, dy: 0, wall: WallIndex::RIGHT1 as Index },
            Offset { dx: 2, dy: 0, wall: WallIndex::UP as Index },
            Offset { dx: 2, dy: -1, wall: WallIndex::LEFT1 as Index },
            Offset { dx: 1, dy: -1, wall: WallIndex::LEFT0 as Index },
            Offset { dx: 0, dy: -1, wall: WallIndex::DOWN as Index },
        ],
        dir: (1, 0),
        span: (
            Angle {
                a: 11.0 * D,
                dx: COS_30,
                dy: -SIN_30,
            },
            Angle {
                a: 15.0 * D,
                dx: 0.0,
                dy: 1.0,
            },
        ),
    },

    UP = {
        corner_wall_offsets: &[
            Offset { dx: 0, dy: -1, wall: WallIndex::LEFT1 as Index },
            Offset { dx: -1, dy: -1, wall: WallIndex::LEFT0 as Index },
            Offset { dx: -2, dy: -1, wall: WallIndex::DOWN as Index },
            Offset { dx: -2, dy: 0, wall: WallIndex::RIGHT0 as Index },
            Offset { dx: -1, dy: 0, wall: WallIndex::RIGHT1 as Index },
        ],
        dir: (0, -1),
        span: (
            Angle {
                a: 7.0 * D,
                dx: -COS_30,
                dy: -SIN_30,
            },
            Angle {
                a: 11.0 * D,
                dx: COS_30,
                dy: -SIN_30,
            },
        ),
    },
    DOWN = {
        corner_wall_offsets: &[
            Offset { dx: 0, dy: 1, wall: WallIndex::RIGHT0 as Index },
            Offset { dx: 1, dy: 1, wall: WallIndex::RIGHT1 as Index },
            Offset { dx: 2, dy: 1, wall: WallIndex::UP as Index },
            Offset { dx: 2, dy: 0, wall: WallIndex::LEFT1 as Index },
            Offset { dx: 1, dy: 0, wall: WallIndex::LEFT0 as Index },
        ],
        dir: (0, 1),
        span: (
            Angle {
                a: 1.0 * D,
                dx: COS_30,
                dy: SIN_30,
            },
            Angle {
                a: 5.0 * D,
                dx: -COS_30,
                dy: SIN_30,
            },
        ),
    }
}

/// The walls for even rows
static ALL0: &[&wall::Wall] = &[&walls::LEFT0, &walls::RIGHT0, &walls::UP];

/// The walls for odd rows
static ALL1: &[&wall::Wall] = &[&walls::LEFT1, &walls::DOWN, &walls::RIGHT1];

/// Returns whether a room is reversed.
///
/// # Arguments
/// *  `pos` - the room position.
fn is_reversed(pos: matrix::Pos) -> bool {
    (pos.col + pos.row) & 1 != 0
}

pub fn minimal_dimensions(width: f32, height: f32) -> (usize, usize) {
    let height = (height.max(VERTICAL_MULTIPLICATOR) / VERTICAL_MULTIPLICATOR)
        .ceil() as usize;

    let width = (width.max(HORIZONTAL_MULTIPLICATOR) / HORIZONTAL_MULTIPLICATOR)
        .floor() as usize;

    (width, height)
}

pub fn back_index(wall: usize) -> usize {
    wall ^ 0b0001
}

pub fn opposite(_pos: WallPos) -> Option<&'static wall::Wall> {
    // There is no opposite wall in a room with an odd number of walls
    None
}

pub fn walls(pos: matrix::Pos) -> &'static [&'static wall::Wall] {
    if is_reversed(pos) {
        &ALL1
    } else {
        &ALL0
    }
}

pub fn center(pos: matrix::Pos) -> physical::Pos {
    physical::Pos {
        x: (pos.col as f32 + 1.0) * HORIZONTAL_MULTIPLICATOR,
        y: (pos.row as f32 + 0.5) * VERTICAL_MULTIPLICATOR
            + if is_reversed(pos) { OFFSET } else { -OFFSET },
    }
}

pub fn room_at(pos: physical::Pos) -> matrix::Pos {
    // Calculate approximations of the room position
    let (i, f) = matrix::partition(pos.y / VERTICAL_MULTIPLICATOR);
    let odd_row = i & 1 == 1;
    let approx_row = i;
    let rel_y = if odd_row { 1.0 - f } else { f };
    let (i, f) = matrix::partition(pos.x / (2.0 * HORIZONTAL_MULTIPLICATOR));
    let approx_col = i * 2;
    let rel_x = f;

    let past_center = rel_x > 0.5;

    matrix::Pos {
        col: approx_col
            + if past_center && 2.0 * (1.0 - rel_x) < rel_y {
                1
            } else if !past_center && 2.0 * rel_x < rel_y {
                -1
            } else {
                0
            },
        row: approx_row,
    }
}

#[cfg(test)]
mod tests {
    use maze_test::maze_test;

    use super::*;
    use crate::test_utils::*;
    use crate::WallPos;

    #[maze_test(tri)]
    fn back(maze: TestMaze) {
        assert_eq!(
            maze.back((matrix_pos(2, 0), &walls::LEFT0)),
            (matrix_pos(1, 0), &walls::RIGHT1)
        );
        assert_eq!(
            maze.back((matrix_pos(2, 0), &walls::RIGHT0)),
            (matrix_pos(3, 0), &walls::LEFT1)
        );
        assert_eq!(
            maze.back((matrix_pos(1, 0), &walls::LEFT1)),
            (matrix_pos(0, 0), &walls::RIGHT0)
        );
        assert_eq!(
            maze.back((matrix_pos(1, 1), &walls::UP)),
            (matrix_pos(1, 0), &walls::DOWN)
        );
        assert_eq!(
            maze.back((matrix_pos(1, 0), &walls::RIGHT1)),
            (matrix_pos(2, 0), &walls::LEFT0)
        );
        assert_eq!(
            maze.back((matrix_pos(1, 0), &walls::DOWN)),
            (matrix_pos(1, 1), &walls::UP)
        );
    }

    #[maze_test(tri)]
    fn corner_walls(maze: TestMaze) {
        assert_eq!(
            maze.corner_walls((matrix_pos(2, 0), &walls::LEFT0))
                .collect::<Vec<_>>(),
            vec![
                (matrix_pos(2, 0), &walls::LEFT0),
                (matrix_pos(1, 0), &walls::DOWN),
                (matrix_pos(1, 1), &walls::RIGHT0),
                (matrix_pos(2, 1), &walls::RIGHT1),
                (matrix_pos(3, 1), &walls::UP),
                (matrix_pos(3, 0), &walls::LEFT1),
            ],
        );
        assert_eq!(
            maze.corner_walls((matrix_pos(2, 0), &walls::RIGHT0))
                .collect::<Vec<_>>(),
            vec![
                (matrix_pos(2, 0), &walls::RIGHT0),
                (matrix_pos(3, 0), &walls::RIGHT1),
                (matrix_pos(4, 0), &walls::UP),
                (matrix_pos(4, -1), &walls::LEFT1),
                (matrix_pos(3, -1), &walls::LEFT0),
                (matrix_pos(2, -1), &walls::DOWN),
            ],
        );
        assert_eq!(
            maze.corner_walls((matrix_pos(1, 0), &walls::LEFT1))
                .collect::<Vec<_>>(),
            vec![
                (matrix_pos(1, 0), &walls::LEFT1),
                (matrix_pos(0, 0), &walls::LEFT0),
                (matrix_pos(-1, 0), &walls::DOWN),
                (matrix_pos(-1, 1), &walls::RIGHT0),
                (matrix_pos(0, 1), &walls::RIGHT1),
                (matrix_pos(1, 1), &walls::UP),
            ],
        );
        assert_eq!(
            maze.corner_walls((matrix_pos(1, 1), &walls::UP))
                .collect::<Vec<_>>(),
            vec![
                (matrix_pos(1, 1), &walls::UP),
                (matrix_pos(1, 0), &walls::LEFT1),
                (matrix_pos(0, 0), &walls::LEFT0),
                (matrix_pos(-1, 0), &walls::DOWN),
                (matrix_pos(-1, 1), &walls::RIGHT0),
                (matrix_pos(0, 1), &walls::RIGHT1),
            ],
        );
        assert_eq!(
            maze.corner_walls((matrix_pos(1, 0), &walls::RIGHT1))
                .collect::<Vec<_>>(),
            vec![
                (matrix_pos(1, 0), &walls::RIGHT1),
                (matrix_pos(2, 0), &walls::UP),
                (matrix_pos(2, -1), &walls::LEFT1),
                (matrix_pos(1, -1), &walls::LEFT0),
                (matrix_pos(0, -1), &walls::DOWN),
                (matrix_pos(0, 0), &walls::RIGHT0),
            ],
        );
        assert_eq!(
            maze.corner_walls((matrix_pos(1, 0), &walls::DOWN))
                .collect::<Vec<_>>(),
            vec![
                (matrix_pos(1, 0), &walls::DOWN),
                (matrix_pos(1, 1), &walls::RIGHT0),
                (matrix_pos(2, 1), &walls::RIGHT1),
                (matrix_pos(3, 1), &walls::UP),
                (matrix_pos(3, 0), &walls::LEFT1),
                (matrix_pos(2, 0), &walls::LEFT0),
            ],
        );
    }

    #[maze_test(tri)]
    fn follow_wall_single_room(maze: TestMaze) {
        assert_eq!(
            vec![
                (matrix_pos(0, 0), &walls::LEFT0),
                (matrix_pos(0, 0), &walls::UP),
                (matrix_pos(0, 0), &walls::RIGHT0),
            ],
            maze.follow_wall((matrix_pos(0, 0), &walls::LEFT0))
                .map(|(from, _)| from)
                .collect::<Vec<WallPos>>()
        );
    }

    #[maze_test(tri)]
    fn follow_wall(mut maze: TestMaze) {
        Navigator::new(&mut maze)
            .from(matrix_pos(1, 0))
            .down(true)
            .right(true)
            .right(true)
            .up(true)
            .left(true);

        assert_eq!(
            vec![
                (matrix_pos(1, 0), &walls::RIGHT1),
                (matrix_pos(2, 0), &walls::LEFT0),
                (matrix_pos(2, 0), &walls::UP),
                (matrix_pos(3, 0), &walls::RIGHT1),
                (matrix_pos(3, 1), &walls::RIGHT0),
                (matrix_pos(2, 1), &walls::DOWN),
                (matrix_pos(1, 1), &walls::LEFT0),
                (matrix_pos(1, 0), &walls::LEFT1),
            ],
            maze.follow_wall((matrix_pos(1, 0), &walls::RIGHT1))
                .map(|(from, _)| from)
                .collect::<Vec<WallPos>>()
        );
    }
}

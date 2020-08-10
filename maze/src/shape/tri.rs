use std::f32::consts::PI;

use crate::WallPos;

use crate::matrix;
use crate::physical;
use crate::wall;

use crate::wall::{Angle, Offset};

use super::{COS_30, SIN_30};

/// A span step angle
const D: f32 = 2.0 * PI / 12.0;

/// The distance between the centre of a room and the centre of a room on the
/// next row.
const HORIZONTAL_MULTIPLICATOR: f32 = COS_30;

/// The distance between the centre of a room and the centre of a room on the
/// next column.
const VERTICAL_MULTIPLICATOR: f32 = 2.0 - 1.0f32 / 2.0f32;

/// The vertical offset for the centre of rooms.
const OFFSET: f32 = 1.0f32 / 4.0f32;

define_shape! {
    << Tri >>

    LEFT0(0) = {
        corner_wall_offsets: &[
            Offset { dx: -1, dy: 0, wall: &DOWN },
            Offset { dx: -1, dy: 1, wall: &RIGHT0 },
            Offset { dx: 0, dy: 1, wall: &RIGHT1 },
            Offset { dx: 1, dy: 1, wall: &UP },
            Offset { dx: 1, dy: 0, wall: &LEFT1 },
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
        previous: &RIGHT0,
        next: &UP,
    },
    RIGHT1(1) = {
        corner_wall_offsets: &[
            Offset { dx: 1, dy: 0, wall: &UP },
            Offset { dx: 1, dy: -1, wall: &LEFT1 },
            Offset { dx: 0, dy: -1, wall: &LEFT0 },
            Offset { dx: -1, dy: -1, wall: &DOWN },
            Offset { dx: -1, dy: 0, wall: &RIGHT0 },
        ],
        dir: (1, 0),
        span: (
            Angle {
                a: 9.0 * D,
                dx: 0.0,
                dy: -1.0,
            },
            Angle {
                a: 1.0 * D,
                dx: COS_30,
                dy: SIN_30,
            },
        ),
        previous: &LEFT1,
        next: &DOWN,
    },

    LEFT1(0) = {
        corner_wall_offsets: &[
            Offset { dx: -1, dy: 0, wall: &LEFT0 },
            Offset { dx: -2, dy: 0, wall: &DOWN },
            Offset { dx: -2, dy: 1, wall: &RIGHT0 },
            Offset { dx: -1, dy: 1, wall: &RIGHT1 },
            Offset { dx: 0, dy: 1, wall: &UP  },
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
        previous: &DOWN,
        next: &RIGHT1,
    },
    RIGHT0(2) = {
        corner_wall_offsets: &[
            Offset { dx: 1, dy: 0, wall: &RIGHT1 },
            Offset { dx: 2, dy: 0, wall: &UP },
            Offset { dx: 2, dy: -1, wall: &LEFT1 },
            Offset { dx: 1, dy: -1, wall: &LEFT0 },
            Offset { dx: 0, dy: -1, wall: &DOWN },
        ],
        dir: (1, 0),
        span: (
            Angle {
                a: 11.0 * D,
                dx: COS_30,
                dy: -SIN_30,
            },
            Angle {
                a: 3.0 * D,
                dx: 0.0,
                dy: 1.0,
            },
        ),
        previous: &UP,
        next: &LEFT0,
    },

    UP(1) = {
        corner_wall_offsets: &[
            Offset { dx: 0, dy: -1, wall: &LEFT1 },
            Offset { dx: -1, dy: -1, wall: &LEFT0 },
            Offset { dx: -2, dy: -1, wall: &DOWN },
            Offset { dx: -2, dy: 0, wall: &RIGHT0 },
            Offset { dx: -1, dy: 0, wall: &RIGHT1 },
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
        previous: &LEFT0,
        next: &RIGHT0,
    },
    DOWN(2) = {
        corner_wall_offsets: &[
            Offset { dx: 0, dy: 1, wall: &RIGHT0 },
            Offset { dx: 1, dy: 1, wall: &RIGHT1 },
            Offset { dx: 2, dy: 1, wall: &UP },
            Offset { dx: 2, dy: 0, wall: &LEFT1 },
            Offset { dx: 1, dy: 0, wall: &LEFT0 },
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
        previous: &RIGHT1,
        next: &LEFT1,
    }
}

/// The walls for even rows
static WALLS_EVEN: &[&wall::Wall] =
    &[&walls::LEFT0, &walls::UP, &walls::RIGHT0];

/// The walls for odd rows
static WALLS_ODD: &[&wall::Wall] =
    &[&walls::LEFT1, &walls::RIGHT1, &walls::DOWN];

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
        &WALLS_ODD
    } else {
        &WALLS_EVEN
    }
}

pub fn cell_to_physical(pos: matrix::Pos) -> physical::Pos {
    physical::Pos {
        x: (pos.col as f32 + 1.0) * HORIZONTAL_MULTIPLICATOR,
        y: (pos.row as f32 + 0.5) * VERTICAL_MULTIPLICATOR
            + if is_reversed(pos) { OFFSET } else { -OFFSET },
    }
}

pub fn physical_to_cell(pos: physical::Pos) -> matrix::Pos {
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

#[cfg_attr(feature = "cargo-clippy", allow(clippy::collapsible_if))]
pub fn physical_to_wall_pos(pos: physical::Pos) -> WallPos {
    let matrix_pos = physical_to_cell(pos);
    let flipped = (matrix_pos.col + matrix_pos.row) & 1 == 1;
    let center = cell_to_physical(matrix_pos);
    let (dx, dy) = (
        pos.x - center.x,
        if flipped {
            center.y - pos.y
        } else {
            pos.y - center.y
        },
    );

    let either = |a, b| if flipped { a } else { b };

    let wall = if dx > 0.0 {
        let t = dx;
        if dy > t * walls::UP.span.0.dy {
            either(&walls::RIGHT1, &walls::RIGHT0)
        } else {
            either(&walls::DOWN, &walls::UP)
        }
    } else {
        let t = -dx;
        if dy > t * walls::UP.span.0.dy {
            either(&walls::LEFT1, &walls::LEFT0)
        } else {
            either(&walls::DOWN, &walls::UP)
        }
    };

    (matrix_pos, wall)
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

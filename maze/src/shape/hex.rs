use std::f32::consts::PI;

use crate::matrix;
use crate::physical;
use crate::wall;

use crate::wall::{Angle, Offset};
use crate::WallPos;

use super::{COS_30, SIN_30};

/// A span step angle
///
/// This is half the angle span used by a single wall.
const D: f32 = 2.0 * PI / 12.0;

/// The distance between the centre of a room and the centre of a room on the
/// next row.
const HORIZONTAL_MULTIPLICATOR: f32 = 2.0 * COS_30;

/// The distance between the centre of a room and the centre of a room on the
/// next column.
const VERTICAL_MULTIPLICATOR: f32 = 2.0 - SIN_30;

/// The height of the top corner.
const TOP_HEIGHT: f32 = 1.0 + SIN_30;

// The walls are arranged in back-to-back pairs
define_shape! {
    << Hex >>

    LEFT0(0) = {
        corner_wall_offsets: &[
            Offset { dx: -1, dy: 0, wall: &DOWN_RIGHT0 },
            Offset { dx: 0, dy: 1, wall: &UP_RIGHT1 },
        ],
        dir: (-1, 0),
        span: (
            Angle {
                a: 5.0 * D,
                dx: -COS_30,
                dy: SIN_30,
            },
            Angle {
                a: 7.0 * D,
                dx: -COS_30,
                dy: -SIN_30,
            },
        ),
        previous: &DOWN_LEFT0,
        next: &UP_LEFT0,
    },
    RIGHT0(3) = {
        corner_wall_offsets: &[
            Offset { dx: 1, dy: 0, wall: &UP_LEFT0 },
            Offset { dx: 1, dy: -1, wall: &DOWN_LEFT1 },
        ],
        dir: (1, 0),
        span: (
            Angle {
                a: 11.0 * D,
                dx: COS_30,
                dy: -SIN_30,
            },
            Angle {
                a: 1.0 * D,
                dx: COS_30,
                dy: SIN_30,
            },
        ),
        previous: &UP_RIGHT0,
        next: &DOWN_RIGHT0,
    },

    LEFT1(0) = {
        corner_wall_offsets: &[
            Offset { dx: -1, dy: 0, wall: &DOWN_RIGHT1 },
            Offset { dx: -1, dy: 1, wall: &UP_RIGHT0 },
        ],
        dir: (-1, 0),
        span: (
            Angle {
                a: 5.0 * D,
                dx: -COS_30,
                dy: SIN_30,
            },
            Angle {
                a: 7.0 * D,
                dx: -COS_30,
                dy: -SIN_30,
            },
        ),
        previous: &DOWN_LEFT1,
        next: &UP_LEFT1,
    },
    RIGHT1(3) = {
        corner_wall_offsets: &[
            Offset { dx: 1, dy: 0, wall: &UP_LEFT1 },
            Offset { dx: 0, dy: -1, wall: &DOWN_LEFT0 },
        ],
        dir: (1, 0),
        span: (
            Angle {
                a: 11.0 * D,
                dx: COS_30,
                dy: -SIN_30,
            },
            Angle {
                a: 1.0 * D,
                dx: COS_30,
                dy: SIN_30,
            },
        ),
        previous: &UP_RIGHT1,
        next: &DOWN_RIGHT1,
    },

    UP_LEFT0(1) = {
        corner_wall_offsets: &[
            Offset { dx: 0, dy: -1, wall: &DOWN_LEFT1 },
            Offset { dx: -1, dy: 0, wall: &UP_RIGHT0 },
        ],
        dir: (0, -1),
        span: (
            Angle {
                a: 7.0 * D,
                dx: -COS_30,
                dy: -SIN_30,
            },
            Angle {
                a: 9.0 * D,
                dx: 0.0,
                dy: -1.0,
            },
        ),
        previous: &LEFT0,
        next: &UP_RIGHT0,
    },
    DOWN_RIGHT1(4) = {
        corner_wall_offsets: &[
            Offset { dx: 0, dy: 1, wall: &UP_RIGHT0 },
            Offset { dx: 1, dy: 0, wall: &LEFT1 },
        ],
        dir: (0, 1),
        span: (
            Angle {
                a: 1.0 * D,
                dx: COS_30,
                dy: SIN_30,
            },
            Angle {
                a: 3.0 * D,
                dx: 0.0,
                dy: 1.0,
            },
        ),
        previous: &RIGHT1,
        next: &DOWN_LEFT1,
    },

    UP_LEFT1(1) = {
        corner_wall_offsets: &[
            Offset { dx: -1, dy: -1, wall: &DOWN_LEFT0 },
            Offset { dx: -1, dy: 0, wall: &RIGHT1 },
        ],
        dir: (-1, -1),
        span: (
            Angle {
                a: 7.0 * D,
                dx: -COS_30,
                dy: -SIN_30,
            },
            Angle {
                a: 9.0 * D,
                dx: 0.0,
                dy: -1.0,
            },
        ),
        previous: &LEFT1,
        next: &UP_RIGHT1,
    },
    DOWN_RIGHT0(4) = {
        corner_wall_offsets: &[
            Offset { dx: 1, dy: 1, wall: &UP_RIGHT1 },
            Offset { dx: 1, dy: 0, wall: &LEFT0 },
        ],
        dir: (1, 1),
        span: (
            Angle {
                a: 1.0 * D,
                dx: COS_30,
                dy: SIN_30,
            },
            Angle {
                a: 3.0 * D,
                dx: 0.0,
                dy: 1.0,
            },
        ),
        previous: &RIGHT0,
        next: &DOWN_LEFT0,
    },

    UP_RIGHT0(2) = {
        corner_wall_offsets: &[
            Offset { dx: 1, dy: -1, wall: &LEFT1 },
            Offset { dx: 0, dy: -1, wall: &DOWN_RIGHT1 },
        ],
        dir: (1, -1),
        span: (
            Angle {
                a: 9.0 * D,
                dx: 0.0,
                dy: -1.0,
            },
            Angle {
                a: 11.0 * D,
                dx: COS_30,
                dy: -SIN_30,
            },
        ),
        previous: &UP_LEFT0,
        next: &RIGHT0,
    },
    DOWN_LEFT1(5) = {
        corner_wall_offsets: &[
            Offset { dx: -1, dy: 1, wall: &RIGHT0 },
            Offset { dx: 0, dy: 1, wall: &UP_LEFT0 },
        ],
        dir: (-1, 1),
        span: (
            Angle {
                a: 3.0 * D,
                dx: 0.0,
                dy: 1.0,
            },
            Angle {
                a: 5.0 * D,
                dx: -COS_30,
                dy: SIN_30,
            },
        ),
        previous: &DOWN_RIGHT1,
        next: &LEFT1,
    },

    UP_RIGHT1(2) = {
        corner_wall_offsets: &[
            Offset { dx: 0, dy: -1, wall: &LEFT0 },
            Offset { dx: -1, dy: -1, wall: &DOWN_RIGHT0 },
        ],
        dir: (0, -1),
        span: (
            Angle {
                a: 9.0 * D,
                dx: 0.0,
                dy: -1.0,
            },
            Angle {
                a: 11.0 * D,
                dx: COS_30,
                dy: -SIN_30,
            },
        ),
        previous: &UP_LEFT1,
        next: &RIGHT1,
    },
    DOWN_LEFT0(5) = {
        corner_wall_offsets: &[
            Offset { dx: 0, dy: 1, wall: &RIGHT1 },
            Offset { dx: 1, dy: 1, wall: &UP_LEFT1 },
        ],
        dir: (0, 1),
        span: (
            Angle {
                a: 3.0 * D,
                dx: 0.0,
                dy: 1.0,
            },
            Angle {
                a: 5.0 * D,
                dx: -COS_30,
                dy: SIN_30,
            },
        ),
        previous: &DOWN_RIGHT0,
        next: &LEFT0,
    }
}

/// The walls for even rows
static ALL0: &[&wall::Wall] = &[
    &walls::LEFT0,
    &walls::UP_LEFT0,
    &walls::UP_RIGHT0,
    &walls::RIGHT0,
    &walls::DOWN_RIGHT0,
    &walls::DOWN_LEFT0,
];

/// The walls for odd rows
static ALL1: &[&wall::Wall] = &[
    &walls::LEFT1,
    &walls::UP_LEFT1,
    &walls::UP_RIGHT1,
    &walls::RIGHT1,
    &walls::DOWN_RIGHT1,
    &walls::DOWN_LEFT1,
];

pub fn minimal_dimensions(width: f32, height: f32) -> (usize, usize) {
    let height = (height.max(VERTICAL_MULTIPLICATOR) / VERTICAL_MULTIPLICATOR)
        .ceil() as usize;

    let hoffset = if height > 1 { 1.0 } else { 0.5 };
    let width = ((width - hoffset).max(HORIZONTAL_MULTIPLICATOR)
        / HORIZONTAL_MULTIPLICATOR)
        .ceil() as usize;

    (width, height)
}

pub fn back_index(wall: usize) -> usize {
    wall ^ 0b0001
}

pub fn opposite(wall_pos: WallPos) -> Option<&'static wall::Wall> {
    let (_, wall) = wall_pos;

    // The left and right walls are back-to-back
    Some(
        walls::ALL[if (wall.index & !0b0011) == 0 {
            wall.index ^ 0b0001
        } else {
            wall.index ^ 0b0011
        }],
    )
}

pub fn walls(pos: matrix::Pos) -> &'static [&'static wall::Wall] {
    if pos.row & 1 == 1 {
        &ALL1
    } else {
        &ALL0
    }
}

pub fn cell_to_physical(pos: matrix::Pos) -> physical::Pos {
    physical::Pos {
        x: (pos.col as f32 + if pos.row & 1 == 1 { 0.5 } else { 1.0 })
            * HORIZONTAL_MULTIPLICATOR,
        y: (pos.row as f32) * VERTICAL_MULTIPLICATOR + 1.0,
    }
}

pub fn physical_to_cell(pos: physical::Pos) -> matrix::Pos {
    // Calculate approximations of the room position
    let (i, f) = matrix::partition(pos.y / VERTICAL_MULTIPLICATOR);
    let odd_row = i & 1 == 1;
    let approx_row = i;
    let rel_y = f;
    let (i, f) = matrix::partition(
        pos.x / (HORIZONTAL_MULTIPLICATOR) - if odd_row { 0.0 } else { 0.5 },
    );
    let approx_col = i;
    let rel_x = f;

    let past_center_x = rel_x > 0.5;
    let corner = if past_center_x {
        rel_x - 0.5
    } else {
        0.5 - rel_x
    } / TOP_HEIGHT
        > rel_y;
    let past_center_y = rel_y > 0.5;

    matrix::Pos {
        col: approx_col
            + if corner && odd_row && !past_center_x {
                -1
            } else if corner && !odd_row && past_center_x {
                1
            } else {
                0
            },
        row: approx_row
            + if corner && !past_center_y {
                -1
            } else if corner && past_center_y {
                1
            } else {
                0
            },
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(clippy::collapsible_if))]
pub fn physical_to_wall_pos(pos: physical::Pos) -> WallPos {
    let matrix_pos = physical_to_cell(pos);
    let odd_row = matrix_pos.row & 1 == 1;
    let center = cell_to_physical(matrix_pos);
    let (dx, dy) = (pos.x - center.x, pos.y - center.y);

    let either = |a, b| if odd_row { a } else { b };

    let wall = if dx > 0.0 {
        if dy < dx * walls::RIGHT0.span.0.dy {
            either(&walls::UP_RIGHT1, &walls::UP_RIGHT0)
        } else if dy > dx * walls::RIGHT0.span.1.dy {
            either(&walls::DOWN_RIGHT1, &walls::DOWN_RIGHT0)
        } else {
            either(&walls::RIGHT1, &walls::RIGHT0)
        }
    } else {
        if dy < dx * walls::LEFT0.span.0.dy {
            either(&walls::UP_LEFT1, &walls::UP_LEFT0)
        } else if dy > dx * walls::LEFT0.span.1.dy {
            either(&walls::DOWN_LEFT1, &walls::DOWN_LEFT0)
        } else {
            either(&walls::LEFT1, &walls::LEFT0)
        }
    };

    (matrix_pos, wall)
}

#[cfg(test)]
mod tests {
    use maze_test::maze_test;

    use super::walls;
    use crate::test_utils::*;
    use crate::WallPos;

    #[maze_test(hex)]
    fn back(maze: TestMaze) {
        assert_eq!(
            maze.back((matrix_pos(1, 0), &walls::LEFT0)),
            (matrix_pos(0, 0), &walls::RIGHT0)
        );
        assert_eq!(
            maze.back((matrix_pos(1, 1), &walls::LEFT1)),
            (matrix_pos(0, 1), &walls::RIGHT1)
        );
        assert_eq!(
            maze.back((matrix_pos(1, 2), &walls::UP_LEFT0)),
            (matrix_pos(1, 1), &walls::DOWN_RIGHT1,)
        );
        assert_eq!(
            maze.back((matrix_pos(1, 1), &walls::UP_LEFT1)),
            (matrix_pos(0, 0), &walls::DOWN_RIGHT0,)
        );
        assert_eq!(
            maze.back((matrix_pos(0, 2), &walls::UP_RIGHT0)),
            (matrix_pos(1, 1), &walls::DOWN_LEFT1,)
        );
        assert_eq!(
            maze.back((matrix_pos(0, 1), &walls::UP_RIGHT1)),
            (matrix_pos(0, 0), &walls::DOWN_LEFT0,)
        );
        assert_eq!(
            maze.back((matrix_pos(0, 0), &walls::RIGHT0)),
            (matrix_pos(1, 0), &walls::LEFT0)
        );
        assert_eq!(
            maze.back((matrix_pos(0, 1), &walls::RIGHT1)),
            (matrix_pos(1, 1), &walls::LEFT1)
        );
        assert_eq!(
            maze.back((matrix_pos(0, 0), &walls::DOWN_RIGHT0)),
            (matrix_pos(1, 1), &walls::UP_LEFT1,)
        );
        assert_eq!(
            maze.back((matrix_pos(0, 1), &walls::DOWN_RIGHT1)),
            (matrix_pos(0, 2), &walls::UP_LEFT0,)
        );
        assert_eq!(
            maze.back((matrix_pos(1, 0), &walls::DOWN_LEFT0)),
            (matrix_pos(1, 1), &walls::UP_RIGHT1,)
        );
        assert_eq!(
            maze.back((matrix_pos(1, 1), &walls::DOWN_LEFT1)),
            (matrix_pos(0, 2), &walls::UP_RIGHT0,)
        );
    }

    #[maze_test(hex)]
    fn opposite(maze: TestMaze) {
        assert_eq!(
            maze.opposite((matrix_pos(1, 0), &walls::LEFT0)).unwrap(),
            &walls::RIGHT0
        );
        assert_eq!(
            maze.opposite((matrix_pos(1, 1), &walls::LEFT1)).unwrap(),
            &walls::RIGHT1
        );
        assert_eq!(
            maze.opposite((matrix_pos(1, 2), &walls::UP_LEFT0)).unwrap(),
            &walls::DOWN_RIGHT0
        );
        assert_eq!(
            maze.opposite((matrix_pos(1, 1), &walls::UP_LEFT1)).unwrap(),
            &walls::DOWN_RIGHT1
        );
        assert_eq!(
            maze.opposite((matrix_pos(0, 2), &walls::UP_RIGHT0))
                .unwrap(),
            &walls::DOWN_LEFT0
        );
        assert_eq!(
            maze.opposite((matrix_pos(0, 1), &walls::UP_RIGHT1))
                .unwrap(),
            &walls::DOWN_LEFT1
        );
        assert_eq!(
            maze.opposite((matrix_pos(0, 0), &walls::RIGHT0)).unwrap(),
            &walls::LEFT0
        );
        assert_eq!(
            maze.opposite((matrix_pos(0, 1), &walls::RIGHT1)).unwrap(),
            &walls::LEFT1
        );
        assert_eq!(
            maze.opposite((matrix_pos(0, 0), &walls::DOWN_RIGHT0))
                .unwrap(),
            &walls::UP_LEFT0
        );
        assert_eq!(
            maze.opposite((matrix_pos(0, 1), &walls::DOWN_RIGHT1))
                .unwrap(),
            &walls::UP_LEFT1
        );
        assert_eq!(
            maze.opposite((matrix_pos(1, 0), &walls::DOWN_LEFT0))
                .unwrap(),
            &walls::UP_RIGHT0
        );
        assert_eq!(
            maze.opposite((matrix_pos(1, 1), &walls::DOWN_LEFT1))
                .unwrap(),
            &walls::UP_RIGHT1
        );
    }

    #[maze_test(hex)]
    fn corner_walls(maze: TestMaze) {
        assert_eq!(
            maze.corner_walls((matrix_pos(1, 2), &walls::LEFT0))
                .collect::<Vec<_>>(),
            vec![
                (matrix_pos(1, 2), &walls::LEFT0),
                (matrix_pos(0, 2), &walls::DOWN_RIGHT0),
                (matrix_pos(1, 3), &walls::UP_RIGHT1),
            ],
        );
        assert_eq!(
            maze.corner_walls((matrix_pos(1, 1), &walls::LEFT1))
                .collect::<Vec<_>>(),
            vec![
                (matrix_pos(1, 1), &walls::LEFT1),
                (matrix_pos(0, 1), &walls::DOWN_RIGHT1),
                (matrix_pos(0, 2), &walls::UP_RIGHT0),
            ],
        );
        assert_eq!(
            maze.corner_walls((matrix_pos(1, 2), &walls::UP_LEFT0))
                .collect::<Vec<_>>(),
            vec![
                (matrix_pos(1, 2), &walls::UP_LEFT0),
                (matrix_pos(1, 1), &walls::DOWN_LEFT1),
                (matrix_pos(0, 2), &walls::UP_RIGHT0),
            ],
        );
        assert_eq!(
            maze.corner_walls((matrix_pos(1, 1), &walls::UP_LEFT1))
                .collect::<Vec<_>>(),
            vec![
                (matrix_pos(1, 1), &walls::UP_LEFT1),
                (matrix_pos(0, 0), &walls::DOWN_LEFT0),
                (matrix_pos(0, 1), &walls::RIGHT1),
            ],
        );
        assert_eq!(
            maze.corner_walls((matrix_pos(1, 2), &walls::UP_RIGHT0))
                .collect::<Vec<_>>(),
            vec![
                (matrix_pos(1, 2), &walls::UP_RIGHT0),
                (matrix_pos(2, 1), &walls::LEFT1),
                (matrix_pos(1, 1), &walls::DOWN_RIGHT1),
            ],
        );
        assert_eq!(
            maze.corner_walls((matrix_pos(1, 1), &walls::UP_RIGHT1))
                .collect::<Vec<_>>(),
            vec![
                (matrix_pos(1, 1), &walls::UP_RIGHT1),
                (matrix_pos(1, 0), &walls::LEFT0),
                (matrix_pos(0, 0), &walls::DOWN_RIGHT0),
            ],
        );
        assert_eq!(
            maze.corner_walls((matrix_pos(1, 2), &walls::RIGHT0))
                .collect::<Vec<_>>(),
            vec![
                (matrix_pos(1, 2), &walls::RIGHT0),
                (matrix_pos(2, 2), &walls::UP_LEFT0),
                (matrix_pos(2, 1), &walls::DOWN_LEFT1),
            ],
        );
        assert_eq!(
            maze.corner_walls((matrix_pos(1, 1), &walls::RIGHT1))
                .collect::<Vec<_>>(),
            vec![
                (matrix_pos(1, 1), &walls::RIGHT1),
                (matrix_pos(2, 1), &walls::UP_LEFT1),
                (matrix_pos(1, 0), &walls::DOWN_LEFT0),
            ],
        );
        assert_eq!(
            maze.corner_walls((matrix_pos(1, 2), &walls::DOWN_RIGHT0))
                .collect::<Vec<_>>(),
            vec![
                (matrix_pos(1, 2), &walls::DOWN_RIGHT0),
                (matrix_pos(2, 3), &walls::UP_RIGHT1),
                (matrix_pos(2, 2), &walls::LEFT0),
            ],
        );
        assert_eq!(
            maze.corner_walls((matrix_pos(1, 1), &walls::DOWN_RIGHT1))
                .collect::<Vec<_>>(),
            vec![
                (matrix_pos(1, 1), &walls::DOWN_RIGHT1),
                (matrix_pos(1, 2), &walls::UP_RIGHT0),
                (matrix_pos(2, 1), &walls::LEFT1),
            ],
        );
        assert_eq!(
            maze.corner_walls((matrix_pos(1, 2), &walls::DOWN_LEFT0))
                .collect::<Vec<_>>(),
            vec![
                (matrix_pos(1, 2), &walls::DOWN_LEFT0),
                (matrix_pos(1, 3), &walls::RIGHT1),
                (matrix_pos(2, 3), &walls::UP_LEFT1),
            ],
        );
        assert_eq!(
            maze.corner_walls((matrix_pos(1, 1), &walls::DOWN_LEFT1))
                .collect::<Vec<_>>(),
            vec![
                (matrix_pos(1, 1), &walls::DOWN_LEFT1),
                (matrix_pos(0, 2), &walls::RIGHT0),
                (matrix_pos(1, 2), &walls::UP_LEFT0),
            ],
        );
    }

    #[maze_test(hex)]
    fn follow_wall_single_room(maze: TestMaze) {
        assert_eq!(
            vec![
                (matrix_pos(0, 0), &walls::LEFT0),
                (matrix_pos(0, 0), &walls::UP_LEFT0),
                (matrix_pos(0, 0), &walls::UP_RIGHT0),
                (matrix_pos(0, 0), &walls::RIGHT0),
                (matrix_pos(0, 0), &walls::DOWN_RIGHT0),
                (matrix_pos(0, 0), &walls::DOWN_LEFT0),
            ],
            maze.follow_wall((matrix_pos(0, 0), &walls::LEFT0))
                .map(|(from, _)| from)
                .collect::<Vec<WallPos>>()
        );
    }

    #[maze_test(hex)]
    fn follow_wall(mut maze: TestMaze) {
        Navigator::new(&mut maze)
            .from(matrix_pos(0, 0))
            .down(true)
            .right(true)
            .up(true);

        assert_eq!(
            vec![
                (matrix_pos(0, 0), &walls::LEFT0),
                (matrix_pos(0, 0), &walls::UP_LEFT0),
                (matrix_pos(0, 0), &walls::UP_RIGHT0),
                (matrix_pos(0, 0), &walls::RIGHT0),
                (matrix_pos(0, 0), &walls::DOWN_RIGHT0),
                (matrix_pos(1, 1), &walls::UP_LEFT1),
                (matrix_pos(1, 0), &walls::LEFT0),
                (matrix_pos(1, 0), &walls::UP_LEFT0),
                (matrix_pos(1, 0), &walls::UP_RIGHT0),
                (matrix_pos(1, 0), &walls::RIGHT0),
                (matrix_pos(1, 0), &walls::DOWN_RIGHT0),
                (matrix_pos(1, 1), &walls::RIGHT1),
                (matrix_pos(1, 1), &walls::DOWN_RIGHT1),
                (matrix_pos(1, 1), &walls::DOWN_LEFT1),
                (matrix_pos(0, 1), &walls::DOWN_RIGHT1),
                (matrix_pos(0, 1), &walls::DOWN_LEFT1),
                (matrix_pos(0, 1), &walls::LEFT1),
                (matrix_pos(0, 1), &walls::UP_LEFT1),
            ],
            maze.follow_wall((matrix_pos(0, 0), &walls::LEFT0))
                .map(|(from, _)| from)
                .collect::<Vec<WallPos>>()
        );
    }
}

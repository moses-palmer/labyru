use std;

use crate::matrix;
use crate::physical;
use crate::wall;

use crate::wall::{Angle, Offset};
use crate::WallPos;

use super::{COS_45, SIN_45};

/// A span step angle
const D: f32 = std::f32::consts::PI / 4.0;

/// The scale factor when converting maze coordinates to physical coordinates
const MULTIPLICATOR: f32 = 2.0 / std::f32::consts::SQRT_2;

define_shape! {
    << Quad >>

    UP(1) = {
        corner_wall_offsets: &[
            Offset { dx: 0, dy: -1, wall: &LEFT  },
            Offset { dx: -1, dy: -1, wall: &DOWN  },
            Offset { dx: -1, dy: 0, wall: &RIGHT  },
        ],
        dir: (0, -1),
        span: (
            Angle {
                a: 5.0 * D,
                dx: -COS_45,
                dy: -SIN_45,
            },
            Angle {
                a: 7.0 * D,
                dx: COS_45,
                dy: -SIN_45,
            },
        ),
        previous: &LEFT,
        next: &RIGHT,
    },
    LEFT(0) = {
        corner_wall_offsets: &[
            Offset { dx: -1, dy: 0, wall: &DOWN  },
            Offset { dx: -1, dy: 1, wall: &RIGHT  },
            Offset { dx: 0, dy: 1, wall: &UP  },
        ],
        dir: (-1, 0),
        span: (
            Angle {
                a: 3.0 * D,
                dx: -COS_45,
                dy: SIN_45,
            },
            Angle {
                a: 5.0 * D,
                dx: -COS_45,
                dy: -SIN_45,
            },
        ),
        previous: &DOWN,
        next: &UP,
    },
    DOWN(3) = {
        corner_wall_offsets: &[
            Offset { dx: 0, dy: 1, wall: &RIGHT  },
            Offset { dx: 1, dy: 1, wall: &UP  },
            Offset { dx: 1, dy: 0, wall: &LEFT  },
        ],
        dir: (0, 1),
        span: (
            Angle {
                a: D,
                dx: COS_45,
                dy: SIN_45,
            },
            Angle {
                a: 3.0 * D,
                dx: -COS_45,
                dy: SIN_45,
            },
        ),
        previous: &RIGHT,
        next: &LEFT,
    },
    RIGHT(2) = {
        corner_wall_offsets: &[
            Offset { dx: 1, dy: 0, wall: &UP  },
            Offset { dx: 1, dy: -1, wall: &LEFT  },
            Offset { dx: 0, dy: -1, wall: &DOWN  },
        ],
        dir: (1, 0),
        span: (
            Angle {
                a: 7.0 * D,
                dx: COS_45,
                dy: -SIN_45,
            },
            Angle {
                a: 1.0 * D,
                dx: COS_45,
                dy: SIN_45,
            },
        ),
        previous: &UP,
        next: &DOWN,
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

pub fn cell_to_physical(pos: matrix::Pos) -> physical::Pos {
    physical::Pos {
        x: (pos.col as f32 + 0.5) * MULTIPLICATOR,
        y: (pos.row as f32 + 0.5) * MULTIPLICATOR,
    }
}

pub fn physical_to_cell(pos: physical::Pos) -> matrix::Pos {
    matrix::Pos {
        col: (pos.x / MULTIPLICATOR).floor() as isize,
        row: (pos.y / MULTIPLICATOR).floor() as isize,
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(clippy::collapsible_if))]
pub fn physical_to_wall_pos(pos: physical::Pos) -> WallPos {
    let matrix_pos = physical_to_cell(pos);
    let center = cell_to_physical(matrix_pos);
    let (dx, dy) = (pos.x - center.x, pos.y - center.y);

    let wall = if dx > dy {
        if dy > -dx {
            &walls::RIGHT
        } else {
            &walls::UP
        }
    } else {
        if dy > -dx {
            &walls::DOWN
        } else {
            &walls::LEFT
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

    #[maze_test(quad)]
    fn back(maze: TestMaze) {
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
    fn opposite(maze: TestMaze) {
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
    fn corner_walls(maze: TestMaze) {
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
    fn follow_wall_single_room(maze: TestMaze) {
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
    fn follow_wall(mut maze: TestMaze) {
        Navigator::new(&mut maze)
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

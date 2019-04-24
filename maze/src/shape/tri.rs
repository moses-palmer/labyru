use std::f32::consts::PI;

use crate::WallPos;

use crate::matrix;
use crate::physical;
use crate::wall;

/// A span step angle
const D: f32 = PI / 6.0;

/// D.cos()
const D_COS: f32 = 0.866_025_4f32;

/// The distance between the centre of a room and the centre of a room on the
/// next row.
const HORIZONTAL_MULTIPLICATOR: f32 = D_COS;

/// The distance between the centre of a room and the centre of a room on the
/// next column.
const VERTICAL_MULTIPLICATOR: f32 = 2.0 - 1.0f32 / 2.0f32;

/// The vertical offset for the centre of rooms.
const OFFSET: f32 = 1.0f32 / 4.0f32;

define_shape! {
    LEFT0 = {
        corner_wall_offsets: &[
            ((-1, 0), WallIndex::DOWN as usize),
            ((-1, 1), WallIndex::RIGHT0 as usize),
            ((0, 1), WallIndex::RIGHT1 as usize),
            ((1, 1), WallIndex::UP as usize),
            ((1, 0), WallIndex::LEFT1 as usize),
        ],
        dir: (-1, 0),
        span: (3.0 * D, 7.0 * D),
    },
    RIGHT1 = {
        corner_wall_offsets: &[
            ((1, 0), WallIndex::UP as usize),
            ((1, -1), WallIndex::LEFT1 as usize),
            ((0, -1), WallIndex::LEFT0 as usize),
            ((-1, -1), WallIndex::DOWN as usize),
            ((-1, 0), WallIndex::RIGHT0 as usize),
        ],
        dir: (1, 0),
        span: (9.0 * D, 13.0 * D),
    },

    LEFT1 = {
        corner_wall_offsets: &[
            ((-1, 0), WallIndex::LEFT0 as usize),
            ((-2, 0), WallIndex::DOWN as usize),
            ((-2, 1), WallIndex::RIGHT0 as usize),
            ((-1, 1), WallIndex::RIGHT1 as usize),
            ((0, 1), WallIndex::UP as usize),
        ],
        dir: (-1, 0),
        span: (5.0 * D, 9.0 * D),
    },
    RIGHT0 = {
        corner_wall_offsets: &[
            ((1, 0), WallIndex::RIGHT1 as usize),
            ((2, 0), WallIndex::UP as usize),
            ((2, -1), WallIndex::LEFT1 as usize),
            ((1, -1), WallIndex::LEFT0 as usize),
            ((0, -1), WallIndex::DOWN as usize),
        ],
        dir: (1, 0),
        span: (11.0 * D, 15.0 * D),
    },

    UP = {
        corner_wall_offsets: &[
            ((0, -1), WallIndex::LEFT1 as usize),
            ((-1, -1), WallIndex::LEFT0 as usize),
            ((-2, -1), WallIndex::DOWN as usize),
            ((-2, 0), WallIndex::RIGHT0 as usize),
            ((-1, 0), WallIndex::RIGHT1 as usize),
        ],
        dir: (0, -1),
        span: (7.0 * D, 11.0 * D),
    },
    DOWN = {
        corner_wall_offsets: &[
            ((0, 1), WallIndex::RIGHT0 as usize),
            ((1, 1), WallIndex::RIGHT1 as usize),
            ((2, 1), WallIndex::UP as usize),
            ((2, 0), WallIndex::LEFT1 as usize),
            ((1, 0), WallIndex::LEFT0 as usize),
        ],
        dir: (0, 1),
        span: (1.0 * D, 5.0 * D),
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
        x: (pos.col as f32 + 0.5) * HORIZONTAL_MULTIPLICATOR,
        y: (pos.row as f32 + 0.5) * VERTICAL_MULTIPLICATOR
            + if is_reversed(pos) { OFFSET } else { -OFFSET },
    }
}

pub fn room_at(pos: physical::Pos) -> matrix::Pos {
    // Calculate approximations of the room position
    let approx_row = (pos.y / VERTICAL_MULTIPLICATOR).floor();
    let row_odd = approx_row as u32 & 1 == 1;
    let approx_col = (pos.x / HORIZONTAL_MULTIPLICATOR).floor();

    // Calculate relative positions within the room
    let rel_y = pos.y - (approx_row * VERTICAL_MULTIPLICATOR);
    let rel_x = pos.x - (approx_col * HORIZONTAL_MULTIPLICATOR);

    if row_odd {
        matrix::Pos {
            col: if rel_x < 0.5 && rel_y > rel_x {
                approx_col as isize - 1
            } else if rel_x > 0.5 && rel_y > rel_x {
                approx_col as isize + 1
            } else {
                approx_col as isize
            },
            row: approx_row as isize,
        }
    } else {
        matrix::Pos {
            col: if rel_x < 0.5 && rel_y < rel_x {
                approx_col as isize - 1
            } else if rel_x > 0.5 && rel_y < rel_x {
                approx_col as isize + 1
            } else {
                approx_col as isize
            },
            row: approx_row as isize,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use crate::Shape;
    use crate::WallPos;

    #[test]
    fn back() {
        let maze = maze(5, 5);

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

    #[test]
    fn corner_walls() {
        let maze = maze(5, 5);

        assert_eq!(
            maze.corner_walls((matrix_pos(2, 0), &walls::LEFT0)),
            vec![
                (matrix_pos(2, 0), &walls::LEFT0),
                (matrix_pos(1, 0), &walls::DOWN),
                (matrix_pos(1, 1), &walls::RIGHT0),
                (matrix_pos(2, 1), &walls::RIGHT1),
                (matrix_pos(3, 1), &walls::UP),
                (matrix_pos(3, 0), &walls::LEFT1),
            ]
        );
        assert_eq!(
            maze.corner_walls((matrix_pos(2, 0), &walls::RIGHT0)),
            vec![
                (matrix_pos(2, 0), &walls::RIGHT0),
                (matrix_pos(3, 0), &walls::RIGHT1),
                (matrix_pos(4, 0), &walls::UP),
                (matrix_pos(4, -1), &walls::LEFT1),
                (matrix_pos(3, -1), &walls::LEFT0),
                (matrix_pos(2, -1), &walls::DOWN),
            ]
        );
        assert_eq!(
            maze.corner_walls((matrix_pos(1, 0), &walls::LEFT1)),
            vec![
                (matrix_pos(1, 0), &walls::LEFT1),
                (matrix_pos(0, 0), &walls::LEFT0),
                (matrix_pos(-1, 0), &walls::DOWN),
                (matrix_pos(-1, 1), &walls::RIGHT0),
                (matrix_pos(0, 1), &walls::RIGHT1),
                (matrix_pos(1, 1), &walls::UP),
            ]
        );
        assert_eq!(
            maze.corner_walls((matrix_pos(1, 1), &walls::UP)),
            vec![
                (matrix_pos(1, 1), &walls::UP),
                (matrix_pos(1, 0), &walls::LEFT1),
                (matrix_pos(0, 0), &walls::LEFT0),
                (matrix_pos(-1, 0), &walls::DOWN),
                (matrix_pos(-1, 1), &walls::RIGHT0),
                (matrix_pos(0, 1), &walls::RIGHT1),
            ]
        );
        assert_eq!(
            maze.corner_walls((matrix_pos(1, 0), &walls::RIGHT1)),
            vec![
                (matrix_pos(1, 0), &walls::RIGHT1),
                (matrix_pos(2, 0), &walls::UP),
                (matrix_pos(2, -1), &walls::LEFT1),
                (matrix_pos(1, -1), &walls::LEFT0),
                (matrix_pos(0, -1), &walls::DOWN),
                (matrix_pos(0, 0), &walls::RIGHT0),
            ]
        );
        assert_eq!(
            maze.corner_walls((matrix_pos(1, 0), &walls::DOWN)),
            vec![
                (matrix_pos(1, 0), &walls::DOWN),
                (matrix_pos(1, 1), &walls::RIGHT0),
                (matrix_pos(2, 1), &walls::RIGHT1),
                (matrix_pos(3, 1), &walls::UP),
                (matrix_pos(3, 0), &walls::LEFT1),
                (matrix_pos(2, 0), &walls::LEFT0),
            ]
        );
    }

    #[test]
    fn follow_wall_single_room() {
        let maze = maze(5, 5);
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

    #[test]
    fn follow_wall() {
        let mut maze = maze(5, 5);

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

    /// Creates a maze.
    ///
    /// # Arguments
    /// *  `width` - The width.
    /// *  `height` - The height.
    fn maze(width: usize, height: usize) -> crate::Maze {
        crate::Maze::new(Shape::Tri, width, height)
    }
}

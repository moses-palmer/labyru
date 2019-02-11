use std::f32::consts::PI;

use super::Shape;
use WallPos;

use matrix;
use physical;
use room;
use wall;

/// A span step angle
const D: f32 = PI / 6.0;

// The walls are arranged in back-to-back pairs
define_walls! {
    LEFT0 = {
        corner_wall_offsets: &[
            ((-1, 0), WallIndex::DOWN_RIGHT0 as usize),
            ((0, 1), WallIndex::UP_RIGHT1 as usize),
        ],
        dir: (-1, 0),
        span: (5.0 * D, 7.0 * D),
    },
    RIGHT0 = {
        corner_wall_offsets: &[
            ((1, 0), WallIndex::UP_LEFT0 as usize),
            ((1, -1), WallIndex::DOWN_LEFT1 as usize),
        ],
        dir: (1, 0),
        span: (11.0 * D, 13.0 * D),
    },

    LEFT1 = {
        corner_wall_offsets: &[
            ((-1, 0), WallIndex::DOWN_RIGHT1 as usize),
            ((-1, 1), WallIndex::UP_RIGHT0 as usize),
        ],
        dir: (-1, 0),
        span: (5.0 * D, 7.0 * D),
    },
    RIGHT1 = {
        corner_wall_offsets: &[
            ((1, 0), WallIndex::UP_LEFT1 as usize),
            ((0, -1), WallIndex::DOWN_LEFT0 as usize),
        ],
        dir: (1, 0),
        span: (11.0 * D, 13.0 * D),
    },

    UP_LEFT0 = {
        corner_wall_offsets: &[
            ((0, -1), WallIndex::DOWN_LEFT1 as usize),
            ((-1, 0), WallIndex::UP_RIGHT0 as usize),
        ],
        dir: (0, -1),
        span: (7.0 * D, 9.0 * D),
    },
    DOWN_RIGHT1 = {
        corner_wall_offsets: &[
            ((0, 1), WallIndex::UP_RIGHT0 as usize),
            ((1, 0), WallIndex::LEFT1 as usize),
        ],
        dir: (0, 1),
        span: (1.0 * D, 3.0 * D),
    },

    UP_LEFT1 = {
        corner_wall_offsets: &[
            ((-1, -1), WallIndex::DOWN_LEFT0 as usize),
            ((-1, 0), WallIndex::RIGHT1 as usize),
        ],
        dir: (-1, -1),
        span: (7.0 * D, 9.0 * D),
    },
    DOWN_RIGHT0 = {
        corner_wall_offsets: &[
            ((1, 1), WallIndex::UP_RIGHT1 as usize),
            ((1, 0), WallIndex::LEFT0 as usize),
        ],
        dir: (1, 1),
        span: (1.0 * D, 3.0 * D),
    },

    UP_RIGHT0 = {
        corner_wall_offsets: &[
            ((1, -1), WallIndex::LEFT1 as usize),
            ((0, -1), WallIndex::DOWN_RIGHT1 as usize),
        ],
        dir: (1, -1),
        span: (9.0 * D, 11.0 * D),
    },
    DOWN_LEFT1 = {
        corner_wall_offsets: &[
            ((-1, 1), WallIndex::RIGHT0 as usize),
            ((0, 1), WallIndex::UP_LEFT0 as usize),
        ],
        dir: (-1, 1),
        span: (3.0 * D, 5.0 * D),
    },

    UP_RIGHT1 = {
        corner_wall_offsets: &[
            ((0, -1), WallIndex::LEFT0 as usize),
            ((-1, -1), WallIndex::DOWN_RIGHT0 as usize),
        ],
        dir: (0, -1),
        span: (9.0 * D, 11.0 * D),
    },
    DOWN_LEFT0 = {
        corner_wall_offsets: &[
            ((0, 1), WallIndex::RIGHT1 as usize),
            ((1, 1), WallIndex::UP_LEFT1 as usize),
        ],
        dir: (0, 1),
        span: (3.0 * D, 5.0 * D),
    }
}

/// The index of the opposite wall.
macro_rules! opposite_index {
    ($wall:expr) => {
        if ($wall & !0b0011) == 0 {
            $wall ^ 0b0001
        } else {
            $wall ^ 0b0011
        }
    };
}

/// The index of the back wall.
macro_rules! back_index {
    ($wall:expr) => {
        $wall ^ 0b0001
    };
}

macro_rules! walls {
    ($pos:expr) => {
        if $pos.1 & 1 == 1 {
            &ALL1
        } else {
            &ALL0
        }
    };
}

/// The walls for even rows
static ALL0: &[&'static wall::Wall] = &[
    &walls::LEFT0,
    &walls::UP_LEFT0,
    &walls::UP_RIGHT0,
    &walls::RIGHT0,
    &walls::DOWN_RIGHT0,
    &walls::DOWN_LEFT0,
];

/// The walls for odd rows
static ALL1: &[&'static wall::Wall] = &[
    &walls::LEFT1,
    &walls::UP_LEFT1,
    &walls::UP_RIGHT1,
    &walls::RIGHT1,
    &walls::DOWN_RIGHT1,
    &walls::DOWN_LEFT1,
];

define_base!(
    horizontal_multiplicator: f32 = 2.0 * (PI / 6.0).cos(),
    vertical_multiplicator: f32 = 2.0 - (PI / 6.0).sin(),
    top_height: f32 = 1.0 - (2.0 * PI - D).sin(),
    gradient: f32 = (1.0 - (2.0 * PI - D).sin()) / (PI / 6.0).cos(),
);

impl Shape for Maze {
    implement_base_shape!();

    fn opposite(&self, wall_pos: WallPos) -> Option<&'static wall::Wall> {
        // The left and right walls are back-to-back
        Some(walls::ALL[opposite_index!(wall_pos.1.index)])
    }
}

impl physical::Physical for Maze {
    fn center(&self, pos: matrix::Pos) -> physical::Pos {
        physical::Pos {
            x: (pos.0 as f32 + if pos.1 & 1 == 1 { 0.5 } else { 1.0 })
                * self.horizontal_multiplicator,
            y: (pos.1 as f32 + 0.5) * self.vertical_multiplicator,
        }
    }

    fn room_at(&self, pos: physical::Pos) -> matrix::Pos {
        // Calculate approximations of the room position
        let approx_row = (pos.y / self.vertical_multiplicator).floor();
        let row_odd = approx_row as i32 & 1 == 1;
        let approx_col = if row_odd {
            (pos.x / self.horizontal_multiplicator)
        } else {
            (pos.x / self.horizontal_multiplicator - 0.5)
        };

        // Calculate relative positions within the room
        let rel_y = pos.y - (approx_row * self.vertical_multiplicator);
        let rel_x = if row_odd {
            (pos.x - ((approx_col - 0.5) * self.horizontal_multiplicator))
        } else {
            (pos.x - (approx_col * self.horizontal_multiplicator))
        };

        if rel_y < (-self.gradient * rel_x) + self.top_height {
            (
                approx_col as isize - !row_odd as isize,
                approx_row as isize - 1,
            )
        } else if rel_y < (self.gradient * rel_x) - self.top_height {
            (
                approx_col as isize + row_odd as isize,
                approx_row as isize - 1,
            )
        } else {
            (approx_col as isize, approx_row as isize)
        }
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
            maze.back(((1, 0), &walls::LEFT0)),
            ((0, 0), &walls::RIGHT0)
        );
        assert_eq!(
            maze.back(((1, 1), &walls::LEFT1)),
            ((0, 1), &walls::RIGHT1)
        );
        assert_eq!(
            maze.back(((1, 2), &walls::UP_LEFT0)),
            ((1, 1), &walls::DOWN_RIGHT1,)
        );
        assert_eq!(
            maze.back(((1, 1), &walls::UP_LEFT1)),
            ((0, 0), &walls::DOWN_RIGHT0,)
        );
        assert_eq!(
            maze.back(((0, 2), &walls::UP_RIGHT0)),
            ((1, 1), &walls::DOWN_LEFT1,)
        );
        assert_eq!(
            maze.back(((0, 1), &walls::UP_RIGHT1)),
            ((0, 0), &walls::DOWN_LEFT0,)
        );
        assert_eq!(
            maze.back(((0, 0), &walls::RIGHT0)),
            ((1, 0), &walls::LEFT0)
        );
        assert_eq!(
            maze.back(((0, 1), &walls::RIGHT1)),
            ((1, 1), &walls::LEFT1)
        );
        assert_eq!(
            maze.back(((0, 0), &walls::DOWN_RIGHT0)),
            ((1, 1), &walls::UP_LEFT1,)
        );
        assert_eq!(
            maze.back(((0, 1), &walls::DOWN_RIGHT1)),
            ((0, 2), &walls::UP_LEFT0,)
        );
        assert_eq!(
            maze.back(((1, 0), &walls::DOWN_LEFT0)),
            ((1, 1), &walls::UP_RIGHT1,)
        );
        assert_eq!(
            maze.back(((1, 1), &walls::DOWN_LEFT1)),
            ((0, 2), &walls::UP_RIGHT0,)
        );
    }

    #[test]
    fn opposite() {
        let maze = Maze::new(5, 5);

        assert_eq!(
            maze.opposite(((1, 0), &walls::LEFT0)).unwrap(),
            &walls::RIGHT0
        );
        assert_eq!(
            maze.opposite(((1, 1), &walls::LEFT1)).unwrap(),
            &walls::RIGHT1
        );
        assert_eq!(
            maze.opposite(((1, 2), &walls::UP_LEFT0)).unwrap(),
            &walls::DOWN_RIGHT0
        );
        assert_eq!(
            maze.opposite(((1, 1), &walls::UP_LEFT1)).unwrap(),
            &walls::DOWN_RIGHT1
        );
        assert_eq!(
            maze.opposite(((0, 2), &walls::UP_RIGHT0)).unwrap(),
            &walls::DOWN_LEFT0
        );
        assert_eq!(
            maze.opposite(((0, 1), &walls::UP_RIGHT1)).unwrap(),
            &walls::DOWN_LEFT1
        );
        assert_eq!(
            maze.opposite(((0, 0), &walls::RIGHT0)).unwrap(),
            &walls::LEFT0
        );
        assert_eq!(
            maze.opposite(((0, 1), &walls::RIGHT1)).unwrap(),
            &walls::LEFT1
        );
        assert_eq!(
            maze.opposite(((0, 0), &walls::DOWN_RIGHT0)).unwrap(),
            &walls::UP_LEFT0
        );
        assert_eq!(
            maze.opposite(((0, 1), &walls::DOWN_RIGHT1)).unwrap(),
            &walls::UP_LEFT1
        );
        assert_eq!(
            maze.opposite(((1, 0), &walls::DOWN_LEFT0)).unwrap(),
            &walls::UP_RIGHT0
        );
        assert_eq!(
            maze.opposite(((1, 1), &walls::DOWN_LEFT1)).unwrap(),
            &walls::UP_RIGHT1
        );
    }

    #[test]
    fn corner_walls() {
        let maze = Maze::new(5, 5);

        assert_eq!(
            maze.corner_walls(((1, 2), &walls::LEFT0)),
            vec![
                ((1, 2), &walls::LEFT0),
                ((0, 2), &walls::DOWN_RIGHT0),
                ((1, 3), &walls::UP_RIGHT1),
            ]
        );
        assert_eq!(
            maze.corner_walls(((1, 1), &walls::LEFT1)),
            vec![
                ((1, 1), &walls::LEFT1),
                ((0, 1), &walls::DOWN_RIGHT1),
                ((0, 2), &walls::UP_RIGHT0),
            ]
        );
        assert_eq!(
            maze.corner_walls(((1, 2), &walls::UP_LEFT0)),
            vec![
                ((1, 2), &walls::UP_LEFT0),
                ((1, 1), &walls::DOWN_LEFT1),
                ((0, 2), &walls::UP_RIGHT0),
            ]
        );
        assert_eq!(
            maze.corner_walls(((1, 1), &walls::UP_LEFT1)),
            vec![
                ((1, 1), &walls::UP_LEFT1),
                ((0, 0), &walls::DOWN_LEFT0),
                ((0, 1), &walls::RIGHT1),
            ]
        );
        assert_eq!(
            maze.corner_walls(((1, 2), &walls::UP_RIGHT0)),
            vec![
                ((1, 2), &walls::UP_RIGHT0),
                ((2, 1), &walls::LEFT1),
                ((1, 1), &walls::DOWN_RIGHT1),
            ]
        );
        assert_eq!(
            maze.corner_walls(((1, 1), &walls::UP_RIGHT1)),
            vec![
                ((1, 1), &walls::UP_RIGHT1),
                ((1, 0), &walls::LEFT0),
                ((0, 0), &walls::DOWN_RIGHT0),
            ]
        );
        assert_eq!(
            maze.corner_walls(((1, 2), &walls::RIGHT0)),
            vec![
                ((1, 2), &walls::RIGHT0),
                ((2, 2), &walls::UP_LEFT0),
                ((2, 1), &walls::DOWN_LEFT1),
            ]
        );
        assert_eq!(
            maze.corner_walls(((1, 1), &walls::RIGHT1)),
            vec![
                ((1, 1), &walls::RIGHT1),
                ((2, 1), &walls::UP_LEFT1),
                ((1, 0), &walls::DOWN_LEFT0),
            ]
        );
        assert_eq!(
            maze.corner_walls(((1, 2), &walls::DOWN_RIGHT0)),
            vec![
                ((1, 2), &walls::DOWN_RIGHT0),
                ((2, 3), &walls::UP_RIGHT1),
                ((2, 2), &walls::LEFT0),
            ]
        );
        assert_eq!(
            maze.corner_walls(((1, 1), &walls::DOWN_RIGHT1)),
            vec![
                ((1, 1), &walls::DOWN_RIGHT1),
                ((1, 2), &walls::UP_RIGHT0),
                ((2, 1), &walls::LEFT1),
            ]
        );
        assert_eq!(
            maze.corner_walls(((1, 2), &walls::DOWN_LEFT0)),
            vec![
                ((1, 2), &walls::DOWN_LEFT0),
                ((1, 3), &walls::RIGHT1),
                ((2, 3), &walls::UP_LEFT1),
            ]
        );
        assert_eq!(
            maze.corner_walls(((1, 1), &walls::DOWN_LEFT1)),
            vec![
                ((1, 1), &walls::DOWN_LEFT1),
                ((0, 2), &walls::RIGHT0),
                ((1, 2), &walls::UP_LEFT0),
            ]
        );
    }

    #[test]
    fn follow_wall_single_room() {
        let maze = Maze::new(5, 5);

        assert_eq!(
            vec![
                ((0, 0), &walls::LEFT0),
                ((0, 0), &walls::UP_LEFT0),
                ((0, 0), &walls::UP_RIGHT0),
                ((0, 0), &walls::RIGHT0),
                ((0, 0), &walls::DOWN_RIGHT0),
                ((0, 0), &walls::DOWN_LEFT0),
            ],
            maze.follow_wall(((0, 0), &walls::LEFT0))
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
                ((0, 0), &walls::LEFT0),
                ((0, 0), &walls::UP_LEFT0),
                ((0, 0), &walls::UP_RIGHT0),
                ((0, 0), &walls::RIGHT0),
                ((0, 0), &walls::DOWN_RIGHT0),
                ((1, 1), &walls::UP_LEFT1),
                ((1, 0), &walls::LEFT0),
                ((1, 0), &walls::UP_LEFT0),
                ((1, 0), &walls::UP_RIGHT0),
                ((1, 0), &walls::RIGHT0),
                ((1, 0), &walls::DOWN_RIGHT0),
                ((1, 1), &walls::RIGHT1),
                ((1, 1), &walls::DOWN_RIGHT1),
                ((1, 1), &walls::DOWN_LEFT1),
                ((0, 1), &walls::DOWN_RIGHT1),
                ((0, 1), &walls::DOWN_LEFT1),
                ((0, 1), &walls::LEFT1),
                ((0, 1), &walls::UP_LEFT1),
            ],
            maze.follow_wall(((0, 0), &walls::LEFT0))
                .map(|(from, _)| from)
                .collect::<Vec<WallPos>>()
        );
    }
}

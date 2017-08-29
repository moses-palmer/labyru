use std::f32::consts::PI;

use super::Shape;
use WallPos;

use matrix;
use physical;
use room;
use wall;


/// A span step angle
const D: f32 = PI / 6.0;

define_walls! {
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

macro_rules! is_reversed {
    ($pos:expr) => {
        ($pos.0 + $pos.1) & 1 != 0
    }
}

/// The index of the back wall.
macro_rules! back_index {
    ($wall:expr) => {
        $wall ^ 0b0001
    }
}

macro_rules! walls {
    ($pos:expr) => {
        if is_reversed!($pos) { &ALL1 } else { &ALL0 }
    }
}


/// The walls for even rows
static ALL0: &[&'static wall::Wall] =
    &[&walls::LEFT0, &walls::RIGHT0, &walls::UP];


/// The walls for odd rows
static ALL1: &[&'static wall::Wall] =
    &[&walls::LEFT1, &walls::DOWN, &walls::RIGHT1];


define_base!(
    horizontal_multiplicator:f32 = (PI / 2.0 + 2.0 * 2.0 * PI / 3.0).cos(),
    vertical_multiplicator: f32 = 2.0 + (PI / 2.0 + 2.0 * PI / 3.0).sin(),
    offset: f32 = 0.5 * (1.0 + (PI / 2.0 + 2.0 * PI / 3.0).sin()),
);


impl Shape for Maze {
    implement_base_shape!();

    fn opposite(&self, _: WallPos) -> Option<&'static wall::Wall> {
        // There is no opposite wall in a room with an odd number of walls
        None
    }
}


impl physical::Physical for Maze {
    fn center(&self, pos: matrix::Pos) -> physical::Pos {
        (
            (pos.0 as f32 + 0.5) * self.horizontal_multiplicator,
            (pos.1 as f32 + 0.5) * self.vertical_multiplicator +
                if is_reversed!(pos) {
                    self.offset
                } else {
                    -self.offset
                },
        )
    }

    fn room_at(&self, _: physical::Pos) -> matrix::Pos {
        unimplemented!();
    }
}


#[cfg(test)]
mod tests {
    use Walkable;
    use super::*;
    use test_utils::*;
    use WallPos;


    #[test]
    fn back() {
        let maze = Maze::new(5, 5);

        assert_eq!(
            maze.back(((2, 0), &walls::LEFT0)),
            ((1, 0), &walls::RIGHT1)
        );
        assert_eq!(
            maze.back(((2, 0), &walls::RIGHT0)),
            ((3, 0), &walls::LEFT1)
        );
        assert_eq!(
            maze.back(((1, 0), &walls::LEFT1)),
            ((0, 0), &walls::RIGHT0)
        );
        assert_eq!(maze.back(((1, 1), &walls::UP)), ((1, 0), &walls::DOWN));
        assert_eq!(
            maze.back(((1, 0), &walls::RIGHT1)),
            ((2, 0), &walls::LEFT0)
        );
        assert_eq!(maze.back(((1, 0), &walls::DOWN)), ((1, 1), &walls::UP));
    }


    #[test]
    fn corner_walls() {
        let maze = Maze::new(5, 5);

        assert_eq!(
            maze.corner_walls(((2, 0), &walls::LEFT0)),
            vec![
                ((2, 0), &walls::LEFT0),
                ((1, 0), &walls::DOWN),
                ((1, 1), &walls::RIGHT0),
                ((2, 1), &walls::RIGHT1),
                ((3, 1), &walls::UP),
                ((3, 0), &walls::LEFT1),
            ]
        );
        assert_eq!(
            maze.corner_walls(((2, 0), &walls::RIGHT0)),
            vec![
                ((2, 0), &walls::RIGHT0),
                ((3, 0), &walls::RIGHT1),
                ((4, 0), &walls::UP),
                ((4, -1), &walls::LEFT1),
                ((3, -1), &walls::LEFT0),
                ((2, -1), &walls::DOWN),
            ]
        );
        assert_eq!(
            maze.corner_walls(((1, 0), &walls::LEFT1)),
            vec![
                ((1, 0), &walls::LEFT1),
                ((0, 0), &walls::LEFT0),
                ((-1, 0), &walls::DOWN),
                ((-1, 1), &walls::RIGHT0),
                ((0, 1), &walls::RIGHT1),
                ((1, 1), &walls::UP),
            ]
        );
        assert_eq!(
            maze.corner_walls(((1, 1), &walls::UP)),
            vec![
                ((1, 1), &walls::UP),
                ((1, 0), &walls::LEFT1),
                ((0, 0), &walls::LEFT0),
                ((-1, 0), &walls::DOWN),
                ((-1, 1), &walls::RIGHT0),
                ((0, 1), &walls::RIGHT1),
            ]
        );
        assert_eq!(
            maze.corner_walls(((1, 0), &walls::RIGHT1)),
            vec![
                ((1, 0), &walls::RIGHT1),
                ((2, 0), &walls::UP),
                ((2, -1), &walls::LEFT1),
                ((1, -1), &walls::LEFT0),
                ((0, -1), &walls::DOWN),
                ((0, 0), &walls::RIGHT0),
            ]
        );
        assert_eq!(
            maze.corner_walls(((1, 0), &walls::DOWN)),
            vec![
                ((1, 0), &walls::DOWN),
                ((1, 1), &walls::RIGHT0),
                ((2, 1), &walls::RIGHT1),
                ((3, 1), &walls::UP),
                ((3, 0), &walls::LEFT1),
                ((2, 0), &walls::LEFT0),
            ]
        );
    }

    #[test]
    fn follow_wall_single_room() {
        let maze = Maze::new(5, 5);
        assert_eq!(
            vec![
                ((0, 0), &walls::LEFT0),
                ((0, 0), &walls::UP),
                ((0, 0), &walls::RIGHT0),
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
            .from((1, 0))
            .down(true)
            .right(true)
            .right(true)
            .up(true)
            .left(true);

        assert_eq!(
            vec![
                ((1, 0), &walls::RIGHT1),
                ((2, 0), &walls::LEFT0),
                ((2, 0), &walls::UP),
                ((3, 0), &walls::RIGHT1),
                ((3, 1), &walls::RIGHT0),
                ((2, 1), &walls::DOWN),
                ((1, 1), &walls::LEFT0),
                ((1, 0), &walls::LEFT1),
            ],
            maze.follow_wall(((1, 0), &walls::RIGHT1))
                .map(|(from, _)| from)
                .collect::<Vec<WallPos>>()
        );
    }
}

use Maze as Base;
use matrix;
use room;
use wall;


define_walls! {
    UP = { dx: 0, dy: -1 },
    LEFT = { dx: -1, dy: 0 },
    DOWN = { dx: 0, dy: 1 },
    RIGHT = { dx: 1, dy: 0 }
}


pub struct Maze {
    rooms: room::Rooms,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Maze {
        Maze { rooms: room::Rooms::new(width, height) }
    }
}

impl Base for Maze {
    #[allow(unused_variables)]
    fn opposite(
        &self,
        pos: matrix::Pos,
        wall: &'static wall::Wall,
    ) -> Option<&'static wall::Wall> {
        Some(
            &walls::ALL[(wall.index + walls::ALL.len() / 2) % walls::ALL.len()],
        )
    }

    #[allow(unused_variables)]
    fn walls(&self, pos: matrix::Pos) -> &'static [&'static wall::Wall] {
        &walls::ALL
    }

    fn rooms(&self) -> &room::Rooms {
        &self.rooms
    }

    fn rooms_mut(&mut self) -> &mut room::Rooms {
        &mut self.rooms
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn back() {
        let maze = Maze::new(5, 5);

        assert_eq!(maze.back((1, 1), &walls::LEFT), ((0, 1), &walls::RIGHT));
        assert_eq!(maze.back((1, 1), &walls::UP), ((1, 0), &walls::DOWN));
        assert_eq!(maze.back((1, 1), &walls::RIGHT), ((2, 1), &walls::LEFT));
        assert_eq!(maze.back((1, 1), &walls::DOWN), ((1, 2), &walls::UP));
    }


    #[test]
    fn opposite() {
        let maze = Maze::new(5, 5);

        assert_eq!(maze.opposite((1, 1), &walls::LEFT).unwrap(), &walls::RIGHT);
        assert_eq!(maze.opposite((1, 1), &walls::UP).unwrap(), &walls::DOWN);
        assert_eq!(maze.opposite((1, 1), &walls::RIGHT).unwrap(), &walls::LEFT);
        assert_eq!(maze.opposite((1, 1), &walls::DOWN).unwrap(), &walls::UP);
    }
}

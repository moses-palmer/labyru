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
    fn opposite(&self,
                pos: matrix::Pos,
                wall: &'static wall::Wall)
                -> Option<&'static wall::Wall> {
        Some(&walls::ALL[(wall.index + walls::ALL.len() / 2) %
                         walls::ALL.len()])
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

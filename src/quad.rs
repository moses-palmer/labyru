use Maze as Base;
use Pos;
use Room;
use Rooms;
use ndarray_rooms;
use wall;


define_walls! {
    UP = { dx: 0, dy: -1 },
    LEFT = { dx: -1, dy: 0 },
    DOWN = { dx: 0, dy: 1 },
    RIGHT = { dx: 1, dy: 0 }
}


pub struct Maze<T: Room> {
    rooms: ndarray_rooms::Rooms<T>,
}

impl<T: Room> Maze<T> {
    pub fn new(width: usize, height: usize) -> Maze<T> {
        Maze { rooms: ndarray_rooms::Rooms::new(width, height) }
    }
}

impl<T: Room> Base<T> for Maze<T> {
    #[allow(unused_variables)]
    fn opposite(&self,
                pos: Pos,
                wall: &'static wall::Wall)
                -> Option<&'static wall::Wall> {
        Some(&walls::ALL[(wall.index + walls::ALL.len() / 2) %
                         walls::ALL.len()])
    }

    #[allow(unused_variables)]
    fn walls(&self, pos: Pos) -> &'static [&'static wall::Wall] {
        &walls::ALL
    }

    fn rooms(&self) -> &Rooms<T> {
        &self.rooms
    }

    fn rooms_mut(&mut self) -> &mut Rooms<T> {
        &mut self.rooms
    }
}

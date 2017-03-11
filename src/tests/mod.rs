use ndarray;

use Rooms;
use room;


pub struct TestMaze {
    rooms: ndarray::Array2<room::Room<u32>>,
}

impl TestMaze {
    pub fn new(width: usize, height: usize) -> TestMaze {
        TestMaze {
            rooms: ndarray::Array2::from_elem((width, height),
                                              room::Room::default()),
        }
    }
}

impl ::Rooms<u32> for TestMaze {
    fn width(&self) -> usize {
        self.rooms.len_of(ndarray::Axis(0))
    }

    fn height(&self) -> usize {
        self.rooms.len_of(ndarray::Axis(1))
    }

    fn get(&self, pos: ::Pos) -> Option<&room::Room<u32>> {
        if self.is_inside(pos) {
            self.rooms.get((pos.0 as usize, pos.1 as usize))
        } else {
            None
        }
    }

    fn get_mut(&mut self, pos: ::Pos) -> Option<&mut room::Room<u32>> {
        if self.is_inside(pos) {
            self.rooms.get_mut((pos.0 as usize, pos.1 as usize))
        } else {
            None
        }
    }
}


#[test]
fn width_correct() {
    let width = 10;
    let height = 5;
    let maze = TestMaze::new(width, height);

    assert!(maze.width() == width);
}


#[test]
fn height_correct() {
    let width = 10;
    let height = 5;
    let maze = TestMaze::new(width, height);

    assert!(maze.height() == height);
}


#[test]
fn is_inside_correct() {
    let width = 10;
    let height = 5;
    let maze = TestMaze::new(width, height);

    assert!(maze.is_inside((0, 0)));
    assert!(maze.is_inside((width as isize - 1, height as isize - 1)));
    assert!(!maze.is_inside((-1, -1)));
    assert!(!maze.is_inside((width as isize, height as isize)));
}

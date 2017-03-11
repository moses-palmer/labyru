use std::collections::HashSet;

use ndarray;

use Maze;
use Pos;
use Rooms;
use room;
use wall;


define_walls! {
    4;

    UP = { dx: 0, dy: -1 },
    LEFT = { dx: -1, dy: 0},
    DOWN = { dx: 0, dy: 1},
    RIGHT = { dx: 1, dy: 0}
}


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

impl ::Maze<u32> for TestMaze {
    #[allow(unused_variables)]
    fn opposite(&self,
                pos: Pos,
                wall: &'static ::wall::Wall)
                -> Option<&'static ::wall::Wall> {
        Some(&walls::ALL[(wall.index + walls::ALL.len() / 2) %
                         walls::ALL.len()])
    }

    #[allow(unused_variables)]
    fn walls(&self, pos: Pos) -> &[&'static wall::Wall] {
        &walls::ALL
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


#[test]
fn can_open() {
    let width = 10;
    let height = 5;
    let mut maze = TestMaze::new(width, height);

    maze.open((0, 0), &walls::DOWN);
    assert!(maze.is_open((0, 0), &walls::DOWN));
    assert!(maze.is_open((0, 1), &walls::UP));
}


#[test]
fn can_close() {
    let width = 10;
    let height = 5;
    let mut maze = TestMaze::new(width, height);

    maze.open((0, 0), &walls::DOWN);
    maze.close((0, 1), &walls::UP);
    assert!(!maze.is_open((0, 0), &walls::DOWN));
    assert!(!maze.is_open((0, 1), &walls::UP));
}


#[test]
fn walls_correct() {
    let width = 10;
    let height = 5;
    let maze = TestMaze::new(width, height);

    let walls = maze.walls((0, 1));
    assert_eq!(walls.iter().cloned().collect::<HashSet<&wall::Wall>>().len(),
               walls.len());
}

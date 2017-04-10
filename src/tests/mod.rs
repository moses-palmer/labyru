use std::collections::HashSet;

use Maze;
use Pos;
use Rooms;
use ndarray_rooms;
use wall;


define_walls! {
    4;

    UP = { dx: 0, dy: -1 },
    LEFT = { dx: -1, dy: 0},
    DOWN = { dx: 0, dy: 1},
    RIGHT = { dx: 1, dy: 0}
}


pub struct TestMaze {
    rooms: ndarray_rooms::Rooms<u32>,
}

impl TestMaze {
    pub fn new(width: usize, height: usize) -> TestMaze {
        TestMaze { rooms: ndarray_rooms::Rooms::new(width, height) }
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

    fn rooms(&self) -> &Rooms<u32> {
        &self.rooms
    }

    fn rooms_mut(&mut self) -> &mut Rooms<u32> {
        &mut self.rooms
    }
}


#[test]
fn width_correct() {
    let width = 10;
    let height = 5;
    let maze = TestMaze::new(width, height);

    assert!(maze.rooms().width() == width);
}


#[test]
fn height_correct() {
    let width = 10;
    let height = 5;
    let maze = TestMaze::new(width, height);

    assert!(maze.rooms().height() == height);
}


#[test]
fn is_inside_correct() {
    let width = 10;
    let height = 5;
    let maze = TestMaze::new(width, height);

    assert!(maze.rooms().is_inside((0, 0)));
    assert!(maze.rooms().is_inside((width as isize - 1, height as isize - 1)));
    assert!(!maze.rooms().is_inside((-1, -1)));
    assert!(!maze.rooms().is_inside((width as isize, height as isize)));
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


#[test]
fn walk_disconnected() {
    let width = 10;
    let height = 5;
    let maze = TestMaze::new(width, height);

    assert!(maze.walk((0, 0), (0, 1)).is_none());
}


#[test]
fn walk_same() {
    let width = 10;
    let height = 5;
    let maze = TestMaze::new(width, height);

    let from = (0, 0);
    let to = (0, 0);
    let expected = vec![(0, 0)];
    assert!(maze.walk(from, to).unwrap().collect::<Vec<Pos>>() == expected);
}


#[test]
fn walk_simple() {
    let width = 10;
    let height = 5;
    let mut maze = TestMaze::new(width, height);

    maze.open((0, 0), &walls::DOWN);

    let from = (0, 0);
    let to = (0, 1);
    let expected = vec![(0, 0), (0, 1)];
    assert!(maze.walk(from, to).unwrap().collect::<Vec<Pos>>() == expected);
}


#[test]
fn walk_shortest() {
    let width = 10;
    let height = 5;
    let mut maze = TestMaze::new(width, height);

    maze.open((0, 0), &walls::DOWN);
    maze.open((0, 1), &walls::DOWN);
    maze.open((0, 2), &walls::DOWN);
    maze.open((0, 2), &walls::RIGHT);
    maze.open((0, 3), &walls::RIGHT);
    maze.open((1, 3), &walls::UP);

    let from = (0, 0);
    let to = (1, 3);
    let expected = vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 3)];
    assert!(maze.walk(from, to).unwrap().collect::<Vec<Pos>>() == expected);
}

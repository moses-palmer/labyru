use ndarray;

use room;


pub struct Rooms<T>
    where T: Clone + Default
{
    rooms: ndarray::Array2<room::Room<T>>,
}

impl<T> Rooms<T>
    where T: Clone + Default
{
    pub fn new(width: usize, height: usize) -> Rooms<T> {
        Rooms {
            rooms: ndarray::Array2::from_elem((width, height),
                                              room::Room::default()),
        }
    }
}

impl<T> ::Rooms<T> for Rooms<T>
    where T: Clone + Default
{
    fn width(&self) -> usize {
        self.rooms.len_of(ndarray::Axis(0))
    }

    fn height(&self) -> usize {
        self.rooms.len_of(ndarray::Axis(1))
    }

    fn get(&self, pos: ::Pos) -> Option<&room::Room<T>> {
        if self.is_inside(pos) {
            self.rooms.get((pos.0 as usize, pos.1 as usize))
        } else {
            None
        }
    }

    fn get_mut(&mut self, pos: ::Pos) -> Option<&mut room::Room<T>> {
        if self.is_inside(pos) {
            self.rooms.get_mut((pos.0 as usize, pos.1 as usize))
        } else {
            None
        }
    }
}

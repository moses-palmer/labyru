use std::collections::HashMap;

use matrix;


/// A maze walker.
///
/// This struct supports walking through a map. From a starting position, it
/// will yield all room positions by mapping a position to the next.
///
/// It will continue until a position maps to `None`. All positions encountered,
/// including `start` and the position yielding `None`, will be returned.
pub struct Walker {
    /// The current position.
    current: matrix::Pos,

    /// Whether `next` should return the next element. This will be true only
    /// for the first call to `next`.
    increment: bool,

    /// The backing map.
    map: HashMap<matrix::Pos, matrix::Pos>,
}


impl Walker {
    /// Creates a walker from a starting position and a supporting map.
    ///
    /// It is possible to walk indefinitely if the mapping contains circular
    /// references.
    pub fn new(start: matrix::Pos,
               map: HashMap<matrix::Pos, matrix::Pos>)
               -> Walker {
        Walker {
            current: start,
            increment: false,
            map: map,
        }
    }
}


impl Iterator for Walker {
    type Item = matrix::Pos;

    fn next(&mut self) -> Option<matrix::Pos> {
        if self.increment {
            match self.map.get(&self.current) {
                Some(next) => {
                    self.current = *next;
                    Some(*next)
                }
                None => None,
            }
        } else {
            self.increment = true;
            Some(self.current)
        }
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn walk_empty() {
        let map = HashMap::new();

        assert_eq!(Walker::new((0, 0), map).collect::<Vec<matrix::Pos>>(),
                   vec![(0, 0)]);
    }


    #[test]
    fn walk_from_unknown() {
        let mut map = HashMap::new();
        map.insert((1, 1), (2, 2));

        assert_eq!(Walker::new((0, 0), map).collect::<Vec<matrix::Pos>>(),
                   vec![(0, 0)]);
    }


    #[test]
    fn walk_path() {
        let mut map = HashMap::new();
        map.insert((1, 1), (2, 2));
        map.insert((2, 2), (2, 3));
        map.insert((2, 3), (2, 4));

        assert_eq!(Walker::new((1, 1), map).collect::<Vec<matrix::Pos>>(),
                   vec![(1, 1), (2, 2), (2, 3), (2, 4)]);
    }
}

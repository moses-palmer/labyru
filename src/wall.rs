use std;


/// A bit mask for a wall.
pub type Mask = u32;


/// A wall.
///
/// Walls have an index, which is used by [Room](../room/struct.Room.html) to
/// generate bit masks, and a direction, which indicates the position of the
/// room on the other side of a wall, relative to the room to which the wall
/// belongs.
#[derive(Clone, PartialEq, PartialOrd)]
pub struct Wall {
    /// The name of this wall.
    pub name: &'static str,

    /// The index of this wall, used to generate the bit mask.
    pub index: usize,

    /// The horizontal offset of the room on the other side of this wall.
    pub dx: isize,

    /// The vertical offset of the room on the other side of this wall.
    pub dy: isize,
}


impl Wall {
    /// The bit mask for this wall.
    pub fn mask(&self) -> Mask {
        1 << self.index
    }
}


impl Eq for Wall {}


impl std::hash::Hash for Wall {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.index.hash(state);
        self.dx.hash(state);
        self.dy.hash(state);
    }
}


impl std::fmt::Debug for Wall {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.write_str(self.name)
    }
}


impl Ord for Wall {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}


/// Defines a wall module.
///
/// This is an internal library macro.
macro_rules! define_walls {
    (
            $( $wall_name:ident = { $( $field:ident: $val:expr, )* } ),* ) => {
        #[allow(unused_imports)]
        pub mod walls {
            use $crate::wall as wall;
            use super::*;

            pub enum WallIndex {
                $($wall_name,)*
            }

            $(pub static $wall_name: wall::Wall = wall::Wall {
                name: stringify!($wall_name),
                index: WallIndex::$wall_name as usize,
                $( $field: $val, )*
            } );*;

            pub static ALL: &[&'static wall::Wall] = &[
                            $(&$wall_name),*];
        }
    }
}

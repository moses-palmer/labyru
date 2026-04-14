/// Dispatches a function call for the current maze to a shape defined module.
///
/// This is an internal library macro.
macro_rules! dispatch {
    ($on:expr => $func:ident ( $($args:ident $(,)?)* ) ) => {
        match $on {
            crate::Shape::Hex => hex::$func($($args,)*),
            crate::Shape::Quad => quad::$func($($args,)*),
            crate::Shape::Tri => tri::$func($($args,)*),
        }
    }
}

/// Defines a wall module.
///
/// This is an internal library macro.
macro_rules! define_shape {
    ( << $name:ident >> $( $wall_name:ident ( $ordinal:expr ) = {
            $( $field:ident: $val:expr, )*
    } ),* ) => {
        #[allow(unused_imports, non_camel_case_types)]
        pub mod walls {
            use $crate::wall as wall;
            use super::*;

            pub enum WallIndex {
                $($wall_name,)*
            }

            $(pub static $wall_name: wall::Wall = wall::Wall {
                name: concat!(stringify!($name), ":", stringify!($wall_name)),
                shape: crate::shape::Shape::$name,
                index: WallIndex::$wall_name as usize,
                ordinal: $ordinal,
                $( $field: $val, )*
            } );*;

            pub static ALL: &[&'static wall::Wall] = &[$(&$wall_name),*];
        }

        /// Returns all walls used in this type of maze.
        pub fn all_walls() -> &'static [&'static wall::Wall] {
            &walls::ALL
        }
    }
}

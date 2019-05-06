/// Defines a full character bitmap.
///
/// The expected format is:
/// ```
/// character!(
///     O X O X O X O X
///     X O X O X O X O
///     O X O X O X O X
///     X O X O X O X O
///     O X O X O X O X
///     X O X O X O X O
///     O X O X O X O X
///     X O X O X O X O
/// )
/// ```
macro_rules! character {
    () => {};
    ($($a:ident $b:ident $c:ident $d:ident $e:ident $f:ident $g:ident $h:ident)*) => {
        [
            $([$a, $b, $c, $d, $e, $f, $g, $h],)*
        ]
    };
}

/// Defines the full mapping from character to bitmap.
///
/// The expected format is:
/// ```
/// let alphabet = alphabet! {
///    'A' => [
///        O X O X O X O X
///        X O X O X O X O
///        O X O X O X O X
///        X O X O X O X O
///        O X O X O X O X
///        X O X O X O X O
///        O X O X O X O X
///        X O X O X O X O
///    ],
///    _ => [
///        O X O X O X O X
///        X O X O X O X O
///        O X O X O X O X
///        X O X O X O X O
///        O X O X O X O X
///        X O X O X O X O
///        O X O X O X O X
///        X O X O X O X O
///    ]
/// }
/// ```
macro_rules! alphabet {
    ($($name:expr => [$($bits:ident)*],)* _ => [$($default:ident)*] ) => {
        {
            let mut map = ::std::collections::hash_map::HashMap::new();
            $(map.insert(
                $name,
                crate::alphabet::Character(character!($($bits)*)),
            );)*
            crate::alphabet::Alphabet {
                default: crate::alphabet::Character(
                    character!($($default)*),
                ),
                map,
            }
        }
    };
}

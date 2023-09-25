/// Defines a full character bitmap.
///
/// The expected format is:
/// ```ignore
/// character!(
///     O X O X O X O X
///     X O X O X O X O
///     O X O X O X O X
///     X O X O X O X O
///     O X O X O X O X
///     X O X O X O X O
///     O X O X O X O X
///     X O X O X O X O
/// );
/// ```
macro_rules! character {
    (O) => {
        false
    };
    (X) => {
        true
    };
    ($($a:ident $b:ident $c:ident $d:ident $e:ident $f:ident $g:ident $h:ident)*) => {
        [
            $([
                character!($a),
                character!($b),
                character!($c),
                character!($d),
                character!($e),
                character!($f),
                character!($g),
                character!($h),
            ],)*
        ]
    };
}

/// Defines the full mapping from character to bitmap.
///
/// The expected format is:
/// ```ignore
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
/// };
/// ```
macro_rules! alphabet {
    ($($name:expr => [$($bits:ident)*],)* _ => [$($default:ident)*] ) => {
        {
            let mut map = ::std::collections::HashMap::new();
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

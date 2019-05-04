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

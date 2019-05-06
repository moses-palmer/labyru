use super::*;

lazy_static! {
    pub static ref ALPHABET: Alphabet = alphabet! {
        ' ' => [
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
        ],
        '!' => [
            O O O X X O O O
            O O O X X O O O
            O O O X X O O O
            O O O X X O O O
            O O O X X O O O
            O O O O O O O O
            O O O X X O O O
            O O O O O O O O
        ],
        '\"' => [
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
        ],
        '#' => [
            O X X O X X O O
            O X X O X X O O
            X X X X X X X O
            O X X O X X O O
            X X X X X X X O
            O X X O X X O O
            O X X O X X O O
            O O O O O O O O
        ],
        '$' => [
            O O O X X O O O
            O O X X X X X O
            O X X O O O O O
            O O X X X X O O
            O O O O O X X O
            O X X X X X O O
            O O O X X O O O
            O O O O O O O O
        ],
        '%' => [
            O O O O O O O O
            X X O O O X X O
            X X O O X X O O
            O O O X X O O O
            O O X X O O O O
            O X X O O X X O
            X X O O O X X O
            O O O O O O O O
        ],
        '&' => [
            O O X X X O O O
            O X X O X X O O
            O O X X X O O O
            O X X X O X X O
            X X O X X X O O
            X X O O X X O O
            O X X X O X X O
            O O O O O O O O
        ],
        '\'' => [
            O O O X X O O O
            O O O X X O O O
            O O X X O O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
        ],
        '(' => [
            O O O O X X O O
            O O O X X O O O
            O O X X O O O O
            O O X X O O O O
            O O X X O O O O
            O O O X X O O O
            O O O O X X O O
            O O O O O O O O
        ],
        ')' => [
            O O X X O O O O
            O O O X X O O O
            O O O O X X O O
            O O O O X X O O
            O O O O X X O O
            O O O X X O O O
            O O X X O O O O
            O O O O O O O O
        ],
        '*' => [
            O O O O O O O O
            O X X O O X X O
            O O X X X X O O
            X X X X X X X X
            O O X X X X O O
            O X X O O X X O
            O O O O O O O O
            O O O O O O O O
        ],
        '+' => [
            O O O O O O O O
            O O O X X O O O
            O O O X X O O O
            O X X X X X X O
            O O O X X O O O
            O O O X X O O O
            O O O O O O O O
            O O O O O O O O
        ],
        ',' => [
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O O O X X O O O
            O O O X X O O O
            O O X X O O O O
        ],
        '-' => [
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O X X X X X X O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
        ],
        '.' => [
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O O O X X O O O
            O O O X X O O O
            O O O O O O O O
        ],
        '/' => [
            O O O O O O X X
            O O O O O X X O
            O O O O X X O O
            O O O X X O O O
            O O X X O O O O
            O X X O O O O O
            X X O O O O O O
            O O O O O O O O
        ],
        '0' => [
            O O X X X X O O
            O X X O O X X O
            O X X O X X X O
            O X X X O X X O
            O X X O O X X O
            O X X O O X X O
            O O X X X X O O
            O O O O O O O O
        ],
        '1' => [
            O O O X X O O O
            O O X X X O O O
            O O O X X O O O
            O O O X X O O O
            O O O X X O O O
            O O O X X O O O
            O X X X X X X O
            O O O O O O O O
        ],
        '2' => [
            O O X X X X O O
            O X X O O X X O
            O O O O X X O O
            O O O X X O O O
            O O X X O O O O
            O X X O O O O O
            O X X X X X X O
            O O O O O O O O
        ],
        '3' => [
            O O X X X X O O
            O X X O O X X O
            O O O O O X X O
            O O O X X X O O
            O O O O O X X O
            O X X O O X X O
            O O X X X X O O
            O O O O O O O O
        ],
        '4' => [
            O O O X X X O O
            O O X X X X O O
            O X X O X X O O
            X X O O X X O O
            X X X X X X X O
            O O O O X X O O
            O O O O X X O O
            O O O O O O O O
        ],
        '5' => [
            O X X X X X X O
            O X X O O O O O
            O X X X X X O O
            O O O O O X X O
            O O O O O X X O
            O X X O O X X O
            O O X X X X O O
            O O O O O O O O
        ],
        '6' => [
            O O O X X X O O
            O O X X O O O O
            O X X O O O O O
            O X X X X X O O
            O X X O O X X O
            O X X O O X X O
            O O X X X X O O
            O O O O O O O O
        ],
        '7' => [
            O X X X X X X O
            O O O O O X X O
            O O O O O X X O
            O O O O X X O O
            O O O X X O O O
            O O O X X O O O
            O O O X X O O O
            O O O O O O O O
        ],
        '8' => [
            O O X X X X O O
            O X X O O X X O
            O X X O O X X O
            O O X X X X O O
            O X X O O X X O
            O X X O O X X O
            O O X X X X O O
            O O O O O O O O
        ],
        '9' => [
            O O X X X X O O
            O X X O O X X O
            O X X O O X X O
            O O X X X X X O
            O O O O O X X O
            O O O O X X O O
            O O X X X O O O
            O O O O O O O O
        ],
        ':' => [
            O O O O O O O O
            O O O X X O O O
            O O O X X O O O
            O O O O O O O O
            O O O O O O O O
            O O O X X O O O
            O O O X X O O O
            O O O O O O O O
        ],
        ';' => [
            O O O O O O O O
            O O O X X O O O
            O O O X X O O O
            O O O O O O O O
            O O O O O O O O
            O O O X X O O O
            O O O X X O O O
            O O X X O O O O
        ],
        '<' => [
            O O O O X X O O
            O O O X X O O O
            O O X X O O O O
            O X X O O O O O
            O O X X O O O O
            O O O X X O O O
            O O O O X X O O
            O O O O O O O O
        ],
        '=' => [
            O O O O O O O O
            O O O O O O O O
            O X X X X X X O
            O O O O O O O O
            O X X X X X X O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
        ],
        '>' => [
            O X X O O O O O
            O O X X O O O O
            O O O X X O O O
            O O O O X X O O
            O O O X X O O O
            O O X X O O O O
            O X X O O O O O
            O O O O O O O O
        ],
        '?' => [
            O O X X X X O O
            O X X O O X X O
            O O O O O X X O
            O O O O X X O O
            O O O X X O O O
            O O O O O O O O
            O O O X X O O O
            O O O O O O O O
        ],
        '@' => [
            O X X X X X O O
            X X O O O X X O
            X X O X X X X O
            X X O X X X X O
            X X O X X X X O
            X X O O O O O O
            O X X X X X O O
            O O O O O O O O
        ],
        'A' => [
            O O O X X O O O
            O O X X X X O O
            O X X O O X X O
            O X X O O X X O
            O X X X X X X O
            O X X O O X X O
            O X X O O X X O
            O O O O O O O O
        ],
        'B' => [
            O X X X X X O O
            O X X O O X X O
            O X X O O X X O
            O X X X X X O O
            O X X O O X X O
            O X X O O X X O
            O X X X X X O O
            O O O O O O O O
        ],
        'C' => [
            O O X X X X O O
            O X X O O X X O
            O X X O O O O O
            O X X O O O O O
            O X X O O O O O
            O X X O O X X O
            O O X X X X O O
            O O O O O O O O
        ],
        'D' => [
            O X X X X O O O
            O X X O X X O O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O X X O X X O O
            O X X X X O O O
            O O O O O O O O
        ],
        'E' => [
            O X X X X X X O
            O X X O O O O O
            O X X O O O O O
            O X X X X X O O
            O X X O O O O O
            O X X O O O O O
            O X X X X X X O
            O O O O O O O O
        ],
        'F' => [
            O X X X X X X O
            O X X O O O O O
            O X X O O O O O
            O X X X X X O O
            O X X O O O O O
            O X X O O O O O
            O X X O O O O O
            O O O O O O O O
        ],
        'G' => [
            O O X X X X O O
            O X X O O X X O
            O X X O O O O O
            O X X O X X X O
            O X X O O X X O
            O X X O O X X O
            O O X X X X X O
            O O O O O O O O
        ],
        'H' => [
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O X X X X X X O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O O O O O O O O
        ],
        'I' => [
            O X X X X X X O
            O O O X X O O O
            O O O X X O O O
            O O O X X O O O
            O O O X X O O O
            O O O X X O O O
            O X X X X X X O
            O O O O O O O O
        ],
        'J' => [
            O O O O O X X O
            O O O O O X X O
            O O O O O X X O
            O O O O O X X O
            O O O O O X X O
            O X X O O X X O
            O O X X X X O O
            O O O O O O O O
        ],
        'K' => [
            X X O O O X X O
            X X O O X X O O
            X X O X X O O O
            X X X X O O O O
            X X O X X O O O
            X X O O X X O O
            X X O O O X X O
            O O O O O O O O
        ],
        'L' => [
            O X X O O O O O
            O X X O O O O O
            O X X O O O O O
            O X X O O O O O
            O X X O O O O O
            O X X O O O O O
            O X X X X X X O
            O O O O O O O O
        ],
        'M' => [
            X X O O O X X O
            X X X O X X X O
            X X X X X X X O
            X X O X O X X O
            X X O O O X X O
            X X O O O X X O
            X X O O O X X O
            O O O O O O O O
        ],
        'N' => [
            X X O O O X X O
            X X X O O X X O
            X X X X O X X O
            X X O X X X X O
            X X O O X X X O
            X X O O O X X O
            X X O O O X X O
            O O O O O O O O
        ],
        'O' => [
            O O X X X X O O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O O X X X X O O
            O O O O O O O O
        ],
        'P' => [
            O X X X X X O O
            O X X O O X X O
            O X X O O X X O
            O X X X X X O O
            O X X O O O O O
            O X X O O O O O
            O X X O O O O O
            O O O O O O O O
        ],
        'Q' => [
            O O X X X X O O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O X X O X X O O
            O O X X O X X O
            O O O O O O O O
        ],
        'R' => [
            O X X X X X O O
            O X X O O X X O
            O X X O O X X O
            O X X X X X O O
            O X X O X X O O
            O X X O O X X O
            O X X O O X X O
            O O O O O O O O
        ],
        'S' => [
            O O X X X X O O
            O X X O O X X O
            O X X O O O O O
            O O X X X X O O
            O O O O O X X O
            O X X O O X X O
            O O X X X X O O
            O O O O O O O O
        ],
        'T' => [
            O X X X X X X O
            O O O X X O O O
            O O O X X O O O
            O O O X X O O O
            O O O X X O O O
            O O O X X O O O
            O O O X X O O O
            O O O O O O O O
        ],
        'U' => [
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O O X X X X O O
            O O O O O O O O
        ],
        'V' => [
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O O X X X X O O
            O O O X X O O O
            O O O O O O O O
        ],
        'W' => [
            X X O O O X X O
            X X O O O X X O
            X X O O O X X O
            X X O X O X X O
            X X X X X X X O
            X X X O X X X O
            X X O O O X X O
            O O O O O O O O
        ],
        'X' => [
            X X O O O O X X
            O X X O O X X O
            O O X X X X O O
            O O O X X O O O
            O O X X X X O O
            O X X O O X X O
            X X O O O O X X
            O O O O O O O O
        ],
        'Y' => [
            X X O O O O X X
            O X X O O X X O
            O O X X X X O O
            O O O X X O O O
            O O O X X O O O
            O O O X X O O O
            O O O X X O O O
            O O O O O O O O
        ],
        'Z' => [
            O X X X X X X O
            O O O O O X X O
            O O O O X X O O
            O O O X X O O O
            O O X X O O O O
            O X X O O O O O
            O X X X X X X O
            O O O O O O O O
        ],
        '[' => [
            O O X X X X O O
            O O X X O O O O
            O O X X O O O O
            O O X X O O O O
            O O X X O O O O
            O O X X O O O O
            O O X X X X O O
            O O O O O O O O
        ],
        '\\' => [
            X X O O O O O O
            O X X O O O O O
            O O X X O O O O
            O O O X X O O O
            O O O O X X O O
            O O O O O X X O
            O O O O O O X X
            O O O O O O O O
        ],
        ']' => [
            O O X X X X O O
            O O O O X X O O
            O O O O X X O O
            O O O O X X O O
            O O O O X X O O
            O O O O X X O O
            O O X X X X O O
            O O O O O O O O
        ],
        '^' => [
            O O O X O O O O
            O O X X X O O O
            O X X O X X O O
            X X O O O X X O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
        ],
        '_' => [
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            X X X X X X X X
        ],
        '`' => [
            O O O X X O O O
            O O O O X X O O
            O O O O O X X O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
        ],
        'a' => [
            O O O O O O O O
            O O O O O O O O
            O O X X X X O O
            O O O O O X X O
            O O X X X X X O
            O X X O O X X O
            O O X X X X X O
            O O O O O O O O
        ],
        'b' => [
            O X X O O O O O
            O X X O O O O O
            O X X X X X O O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O X X X X X O O
            O O O O O O O O
        ],
        'c' => [
            O O O O O O O O
            O O O O O O O O
            O O X X X X O O
            O X X O O O O O
            O X X O O O O O
            O X X O O O O O
            O O X X X X O O
            O O O O O O O O
        ],
        'd' => [
            O O O O O X X O
            O O O O O X X O
            O O X X X X X O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O O X X X X X O
            O O O O O O O O
        ],
        'e' => [
            O O O O O O O O
            O O O O O O O O
            O O X X X X O O
            O X X O O X X O
            O X X X X X X O
            O X X O O O O O
            O O X X X X O O
            O O O O O O O O
        ],
        'f' => [
            O O O X X X O O
            O O X X O O O O
            O X X X X X O O
            O O X X O O O O
            O O X X O O O O
            O O X X O O O O
            O O X X O O O O
            O O O O O O O O
        ],
        'g' => [
            O O O O O O O O
            O O O O O O O O
            O O X X X X X O
            O X X O O X X O
            O X X O O X X O
            O O X X X X X O
            O O O O O X X O
            O X X X X X O O
        ],
        'h' => [
            O X X O O O O O
            O X X O O O O O
            O X X X X X O O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O O O O O O O O
        ],
        'i' => [
            O O O X X O O O
            O O O O O O O O
            O O X X X O O O
            O O O X X O O O
            O O O X X O O O
            O O O X X O O O
            O O O X X X X O
            O O O O O O O O
        ],
        'j' => [
            O O O O X X O O
            O O O O O O O O
            O O O O X X O O
            O O O O X X O O
            O O O O X X O O
            O O O O X X O O
            O O O O X X O O
            O X X X X O O O
        ],
        'k' => [
            O X X O O O O O
            O X X O O O O O
            O X X O O X X O
            O X X O X X O O
            O X X X X O O O
            O X X O X X O O
            O X X O O X X O
            O O O O O O O O
        ],
        'l' => [
            O O X X X O O O
            O O O X X O O O
            O O O X X O O O
            O O O X X O O O
            O O O X X O O O
            O O O X X O O O
            O O O X X X X O
            O O O O O O O O
        ],
        'm' => [
            O O O O O O O O
            O O O O O O O O
            X X O O X X O O
            X X X X X X X O
            X X O X O X X O
            X X O X O X X O
            X X O O O X X O
            O O O O O O O O
        ],
        'n' => [
            O O O O O O O O
            O O O O O O O O
            O X X X X X O O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O O O O O O O O
        ],
        'o' => [
            O O O O O O O O
            O O O O O O O O
            O O X X X X O O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O O X X X X O O
            O O O O O O O O
        ],
        'p' => [
            O O O O O O O O
            O O O O O O O O
            O X X X X X O O
            O X X O O X X O
            O X X O O X X O
            O X X X X X O O
            O X X O O O O O
            O X X O O O O O
        ],
        'q' => [
            O O O O O O O O
            O O O O O O O O
            O O X X X X X O
            O X X O O X X O
            O X X O O X X O
            O O X X X X X O
            O O O O O X X O
            O O O O O X X O
        ],
        'r' => [
            O O O O O O O O
            O O O O O O O O
            O X X X X X O O
            O X X O O X X O
            O X X O O O O O
            O X X O O O O O
            O X X O O O O O
            O O O O O O O O
        ],
        's' => [
            O O O O O O O O
            O O O O O O O O
            O O X X X X X O
            O X X O O O O O
            O O X X X X O O
            O O O O O X X O
            O X X X X X O O
            O O O O O O O O
        ],
        't' => [
            O O X X O O O O
            O O X X O O O O
            O X X X X X X O
            O O X X O O O O
            O O X X O O O O
            O O X X O O O O
            O O O X X X X O
            O O O O O O O O
        ],
        'u' => [
            O O O O O O O O
            O O O O O O O O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O O X X X X X O
            O O O O O O O O
        ],
        'v' => [
            O O O O O O O O
            O O O O O O O O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O O X X X X O O
            O O O X X O O O
            O O O O O O O O
        ],
        'w' => [
            O O O O O O O O
            O O O O O O O O
            X X O O O X X O
            X X O O O X X O
            X X O X O X X O
            O X X X X X O O
            O X X O X X O O
            O O O O O O O O
        ],
        'x' => [
            O O O O O O O O
            O O O O O O O O
            X X O O O X X O
            O X X O X X O O
            O O X X X O O O
            O X X O X X O O
            X X O O O X X O
            O O O O O O O O
        ],
        'y' => [
            O O O O O O O O
            O O O O O O O O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O O X X X X X O
            O O O O O X X O
            O O X X X X O O
        ],
        'z' => [
            O O O O O O O O
            O O O O O O O O
            O X X X X X X O
            O O O O X X O O
            O O O X X O O O
            O O X X O O O O
            O X X X X X X O
            O O O O O O O O
        ],
        '{' => [
            O O O O X X X O
            O O O X X O O O
            O O O X X O O O
            O X X X O O O O
            O O O X X O O O
            O O O X X O O O
            O O O O X X X O
            O O O O O O O O
        ],
        '|' => [
            O O O X X O O O
            O O O X X O O O
            O O O X X O O O
            O O O X X O O O
            O O O X X O O O
            O O O X X O O O
            O O O X X O O O
            O O O O O O O O
        ],
        '}' => [
            O X X X O O O O
            O O O X X O O O
            O O O X X O O O
            O O O O X X X O
            O O O X X O O O
            O O O X X O O O
            O X X X O O O O
            O O O O O O O O
        ],
        '~' => [
            O X X X O X X O
            X X O X X X O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
        ],
        '¡' => [
            O O O X X O O O
            O O O O O O O O
            O O O X X O O O
            O O O X X O O O
            O O O X X O O O
            O O O X X O O O
            O O O X X O O O
            O O O O O O O O
        ],
        '£' => [
            O O X X X O O O
            O X X O X X O O
            O X X O O O O O
            X X X X O O O O
            O X X O O O O O
            O X X O O X X O
            X X X X X X O O
            O O O O O O O O
        ],
        '¥' => [
            X X O O O O X X
            O X X O O X X O
            O O X X X X O O
            O O O X X O O O
            O O X X X X O O
            O O O X X O O O
            O O O X X O O O
            O O O O O O O O
        ],
        '°' => [
            O O X X X X O O
            O X X O O X X O
            O O X X X X O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
        ],
        '±' => [
            O O O X X O O O
            O O O X X O O O
            O X X X X X X O
            O O O X X O O O
            O O O X X O O O
            O O O O O O O O
            O X X X X X X O
            O O O O O O O O
        ],
        '²' => [
            O X X X O O O O
            O O O X X O O O
            O O X X O O O O
            O X X O O O O O
            O X X X X O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
        ],
        '³' => [
            O X X X X O O O
            O O O O X X O O
            O O O X X O O O
            O O O O X X O O
            O X X X X O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
        ],
        'µ' => [
            O O O O O O O O
            O O O O O O O O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O X X X X X O O
            O X X O O O O O
            X X O O O O O O
        ],
        '¹' => [
            O O X X O O O O
            O X X X O O O O
            O O X X O O O O
            O O X X O O O O
            O O X X O O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
        ],
        '¿' => [
            O O O X X O O O
            O O O O O O O O
            O O O X X O O O
            O O X X O O O O
            O X X O O O O O
            O X X O O X X O
            O O X X X X O O
            O O O O O O O O
        ],
        'À' => [
            O X X X O O O O
            O O O O O O O O
            O O X X X X O O
            O X X O O X X O
            O X X X X X X O
            O X X O O X X O
            O X X O O X X O
            O O O O O O O O
        ],
        'Á' => [
            O O O O X X X O
            O O O O O O O O
            O O X X X X O O
            O X X O O X X O
            O X X X X X X O
            O X X O O X X O
            O X X O O X X O
            O O O O O O O O
        ],
        'Â' => [
            O O O X X O O O
            O X X O O X X O
            O O O O O O O O
            O O X X X X O O
            O X X O O X X O
            O X X X X X X O
            O X X O O X X O
            O O O O O O O O
        ],
        'Ã' => [
            O X X X O X X O
            X X O X X X O O
            O O O O O O O O
            O O X X X X O O
            O X X O O X X O
            O X X X X X X O
            O X X O O X X O
            O O O O O O O O
        ],
        'Ä' => [
            O X X O O X X O
            O O O O O O O O
            O O X X X X O O
            O X X O O X X O
            O X X X X X X O
            O X X O O X X O
            O X X O O X X O
            O O O O O O O O
        ],
        'Å' => [
            O O O X X O O O
            O O O X X O O O
            O O O O O O O O
            O O X X X X O O
            O X X O O X X O
            O X X X X X X O
            O X X O O X X O
            O O O O O O O O
        ],
        'Æ' => [
            O O X X X X X X
            O X X O X X O O
            X X O O X X O O
            X X X X X X X O
            X X O O X X O O
            X X O O X X O O
            X X O O X X X X
            O O O O O O O O
        ],
        'Ç' => [
            O O X X X X O O
            O X X O O X X O
            O X X O O O O O
            O X X O O O O O
            O X X O O O O O
            O X X O O X X O
            O O X X X X O O
            O O O X X O O O
        ],
        'È' => [
            O X X X O O O O
            O O O O O O O O
            X X X X X X X O
            X X O O O O O O
            X X X X X O O O
            X X O O O O O O
            X X X X X X X O
            O O O O O O O O
        ],
        'É' => [
            O O O O X X X O
            O O O O O O O O
            X X X X X X X O
            X X O O O O O O
            X X X X X O O O
            X X O O O O O O
            X X X X X X X O
            O O O O O O O O
        ],
        'Ê' => [
            O O O X X O O O
            O X X O O X X O
            O O O O O O O O
            X X X X X X X O
            X X X X O O O O
            X X O O O O O O
            X X X X X X X O
            O O O O O O O O
        ],
        'Ë' => [
            O X X O O X X O
            O O O O O O O O
            X X X X X X X O
            X X O O O O O O
            X X X X X O O O
            X X O O O O O O
            X X X X X X X O
            O O O O O O O O
        ],
        'Ì' => [
            O X X X O O O O
            O O O O O O O O
            O X X X X X X O
            O O O X X O O O
            O O O X X O O O
            O O O X X O O O
            O X X X X X X O
            O O O O O O O O
        ],
        'Í' => [
            O O O O X X X O
            O O O O O O O O
            O X X X X X X O
            O O O X X O O O
            O O O X X O O O
            O O O X X O O O
            O X X X X X X O
            O O O O O O O O
        ],
        'Î' => [
            O O O X X O O O
            O X X O O X X O
            O O O O O O O O
            O X X X X X X O
            O O O X X O O O
            O O O X X O O O
            O X X X X X X O
            O O O O O O O O
        ],
        'Ï' => [
            O X X O O X X O
            O O O O O O O O
            O X X X X X X O
            O O O X X O O O
            O O O X X O O O
            O O O X X O O O
            O X X X X X X O
            O O O O O O O O
        ],
        'Ð' => [
            O X X X X O O O
            O X X O X X O O
            O X X O O X X O
            X X X X O X X O
            O X X O O X X O
            O X X O X X O O
            O X X X X O O O
            O O O O O O O O
        ],
        'Ñ' => [
            O X X X O X X O
            X X O X X X O O
            O O O O O O O O
            X X O O O X X O
            X X X X O X X O
            X X O X X X X O
            X X O O O X X O
            O O O O O O O O
        ],
        'Ò' => [
            O X X X O O O O
            O O O O O O O O
            O X X X X X O O
            X X O O O X X O
            X X O O O X X O
            X X O O O X X O
            O X X X X X O O
            O O O O O O O O
        ],
        'Ó' => [
            O O O O X X X O
            O O O O O O O O
            O X X X X X O O
            X X O O O X X O
            X X O O O X X O
            X X O O O X X O
            O X X X X X O O
            O O O O O O O O
        ],
        'Ô' => [
            O O O X X O O O
            O X X O O X X O
            O O O O O O O O
            O X X X X X O O
            X X O O O X X O
            X X O O O X X O
            O X X X X X O O
            O O O O O O O O
        ],
        'Õ' => [
            O X X X O X X O
            X X O X X X O O
            O O O O O O O O
            O X X X X X O O
            X X O O O X X O
            X X O O O X X O
            O X X X X X O O
            O O O O O O O O
        ],
        'Ö' => [
            O X X O O X X O
            O O O O O O O O
            O X X X X X O O
            X X O O O X X O
            X X O O O X X O
            X X O O O X X O
            O X X X X X O O
            O O O O O O O O
        ],
        '×' => [
            O O O O O O O O
            X X O O O X X O
            O X X O X X O O
            O O X X X O O O
            O X X O X X O O
            X X O O O X X O
            O O O O O O O O
            O O O O O O O O
        ],
        'Ø' => [
            O O X X X X X O
            O X X O O X X O
            O X X O X X X O
            O X X X X X X O
            O X X X O X X O
            O X X O O X X O
            O X X X X X O O
            O O O O O O O O
        ],
        'Ù' => [
            O X X X O O O O
            O O O O O O O O
            X X O O O X X O
            X X O O O X X O
            X X O O O X X O
            X X O O O X X O
            O X X X X X O O
            O O O O O O O O
        ],
        'Ú' => [
            O O O O X X X O
            O O O O O O O O
            X X O O O X X O
            X X O O O X X O
            X X O O O X X O
            X X O O O X X O
            O X X X X X O O
            O O O O O O O O
        ],
        'Û' => [
            O O O X X O O O
            O X X O O X X O
            O O O O O O O O
            X X O O O X X O
            X X O O O X X O
            X X O O O X X O
            O X X X X X O O
            O O O O O O O O
        ],
        'Ü' => [
            O X X O O X X O
            O O O O O O O O
            X X O O O X X O
            X X O O O X X O
            X X O O O X X O
            X X O O O X X O
            O X X X X X O O
            O O O O O O O O
        ],
        'Ý' => [
            O O O O X X X O
            O O O O O O O O
            O X X O O X X O
            O X X O O X X O
            O O X X X X O O
            O O O X X O O O
            O O O X X O O O
            O O O O O O O O
        ],
        'Þ' => [
            X X O O O O O O
            X X O O O O O O
            X X X X X X O O
            X X O O O X X O
            X X X X X X O O
            X X O O O O O O
            X X O O O O O O
            O O O O O O O O
        ],
        'ß' => [
            O O X X X X O O
            O X X O O X X O
            O X X O O X X O
            O X X O X X O O
            O X X O O X X O
            O X X O O X X O
            O X X O X X O O
            O O O O O O O O
        ],
        'à' => [
            O X X X O O O O
            O O O O O O O O
            O O X X X X O O
            O O O O O X X O
            O O X X X X X O
            O X X O O X X O
            O O X X X X X O
            O O O O O O O O
        ],
        'á' => [
            O O O O X X X O
            O O O O O O O O
            O O X X X X O O
            O O O O O X X O
            O O X X X X X O
            O X X O O X X O
            O O X X X X X O
            O O O O O O O O
        ],
        'â' => [
            O O O X X O O O
            O X X O O X X O
            O O O O O O O O
            O O X X X X X O
            O X X O O X X O
            X X O O O X X O
            O X X X X X X O
            O O O O O O O O
        ],
        'ã' => [
            O X X X O X X O
            X X O X X X O O
            O O O O O O O O
            O O X X X X X O
            O X X O O X X O
            X X O O O X X O
            O X X X X X X O
            O O O O O O O O
        ],
        'ä' => [
            O X X O O X X O
            O O O O O O O O
            O O X X X X O O
            O O O O O X X O
            O O X X X X X O
            O X X O O X X O
            O O X X X X X O
            O O O O O O O O
        ],
        'å' => [
            O O O X X O O O
            O O O X X O O O
            O O O O O O O O
            O O X X X X X O
            O X X O O X X O
            X X O O O X X O
            O X X X X X X O
            O O O O O O O O
        ],
        'æ' => [
            O O O O O O O O
            O O O O O O O O
            O X X X X X X O
            O O O X X O X X
            O X X X X X X X
            X X O X X O O O
            O X X X O X X X
            O O O O O O O O
        ],
        'ç' => [
            O O O O O O O O
            O O O O O O O O
            O O X X X X O O
            O X X O O O O O
            O X X O O O O O
            O X X O O O O O
            O O X X X X O O
            O O O X X O O O
        ],
        'è' => [
            O X X X O O O O
            O O O O O O O O
            O O X X X X O O
            O X X O O X X O
            O X X X X X X O
            O X X O O O O O
            O O X X X X O O
            O O O O O O O O
        ],
        'é' => [
            O O O O X X X O
            O O O O O O O O
            O O X X X X O O
            O X X O O X X O
            O X X X X X X O
            O X X O O O O O
            O O X X X X O O
            O O O O O O O O
        ],
        'ê' => [
            O O O X X O O O
            O X X O O X X O
            O O O O O O O O
            O O X X X X O O
            O X X X X X X O
            O X X O O O O O
            O O X X X X O O
            O O O O O O O O
        ],
        'ë' => [
            O X X O O X X O
            O O O O O O O O
            O O X X X X O O
            O X X O O X X O
            O X X X X X X O
            O X X O O O O O
            O O X X X X O O
            O O O O O O O O
        ],
        'ì' => [
            O X X X O O O O
            O O O O O O O O
            O O X X X O O O
            O O O X X O O O
            O O O X X O O O
            O O O X X O O O
            O O X X X X O O
            O O O O O O O O
        ],
        'í' => [
            O O O O X X X O
            O O O O O O O O
            O O X X X O O O
            O O O X X O O O
            O O O X X O O O
            O O O X X O O O
            O O X X X X O O
            O O O O O O O O
        ],
        'î' => [
            O O O X X O O O
            O X X O O X X O
            O O O O O O O O
            O O X X X O O O
            O O O X X O O O
            O O O X X O O O
            O O X X X X O O
            O O O O O O O O
        ],
        'ï' => [
            O X X O O X X O
            O O O O O O O O
            O O X X X O O O
            O O O X X O O O
            O O O X X O O O
            O O O X X O O O
            O O X X X X O O
            O O O O O O O O
        ],
        'ð' => [
            O O O O X X O O
            O O X X X X X O
            O O O O X X O O
            O X X X X X O O
            X X O O X X O O
            X X O O X X O O
            O X X X X O O O
            O O O O O O O O
        ],
        'ñ' => [
            O X X X O X X O
            X X O X X X O O
            O O O O O O O O
            O X X X X X O O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O O O O O O O O
        ],
        'ò' => [
            O X X X O O O O
            O O O O O O O O
            O O X X X X O O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O O X X X X O O
            O O O O O O O O
        ],
        'ó' => [
            O O O O X X X O
            O O O O O O O O
            O O X X X X O O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O O X X X X O O
            O O O O O O O O
        ],
        'ô' => [
            O O O X X O O O
            O X X O O X X O
            O O O O O O O O
            O O X X X X O O
            O X X O O X X O
            O X X O O X X O
            O O X X X X O O
            O O O O O O O O
        ],
        'õ' => [
            O X X X O X X O
            X X O X X X O O
            O O O O O O O O
            O O X X X X O O
            O X X O O X X O
            O X X O O X X O
            O O X X X X O O
            O O O O O O O O
        ],
        'ö' => [
            O X X O O X X O
            O O O O O O O O
            O O X X X X O O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O O X X X X O O
            O O O O O O O O
        ],
        '÷' => [
            O O O X X O O O
            O O O X X O O O
            O O O O O O O O
            O X X X X X X O
            O O O O O O O O
            O O O X X O O O
            O O O X X O O O
            O O O O O O O O
        ],
        'ø' => [
            O O O O O O O O
            O O O O O O X O
            O X X X X X O O
            X X O O X X X O
            X X O X O X X O
            X X X O O X X O
            O X X X X X O O
            X O O O O O O O
        ],
        'ù' => [
            O X X X O O O O
            O O O O O O O O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O O X X X X X O
            O O O O O O O O
        ],
        'ú' => [
            O O O O X X X O
            O O O O O O O O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O O X X X X X O
            O O O O O O O O
        ],
        'û' => [
            O O O X X O O O
            O X X O O X X O
            O O O O O O O O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O O X X X X X O
            O O O O O O O O
        ],
        'ü' => [
            O X X O O X X O
            O O O O O O O O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O O X X X X X O
            O O O O O O O O
        ],
        'ý' => [
            O O O O X X X O
            O O O O O O O O
            O X X O O X X O
            O X X O O X X O
            O X X O O X X O
            O O X X X X X O
            O O O O O X X O
            O O X X X X O O
        ],
        'þ' => [
            O X X O O O O O
            O X X O O O O O
            O X X X X X O O
            O X X O O X X O
            O X X O O X X O
            O X X X X X O O
            O X X O O O O O
            O X X O O O O O
        ],
        'ÿ' => [
            O O O O O O O O
            O O O O O O O O
            O X X X O X X O
            X X O X X O X X
            X X O X X O X X
            O X X O X X X O
            O O O O O O O O
            O O O O O O O O
        ],
        _ => [
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
            O O O O O O O O
        ]
    };
}

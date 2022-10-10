/// Encodes zero. Works because `+` coerces `[]` into an integer type: `0`.
const ZERO: &str = "+[]";

/// Encodes one. Works because `!` coerces `[]` into a boolean type (`false`),
/// and then the next `!` coerces it into `true`. The final `+` coerces it
/// into an integer type, which is `1`, as the value is `true`.
const ONE: &str = "+!![]";

/// Encodes true. Works becuase `+[]` is zero, and `!` coerces it into `true`
/// (as `0` is `false`, so `!0` is `true`).
const TRUE: &str = "!+[]";

/// Encodes false. Works because `!` coerces `[]` into a boolean type (`false`).
const FALSE: &str = "![]";

/// Encodes any number into a JSFuck number
pub fn encode_number(n: isize) -> String {
    if n == 0 {
        ZERO.into()
    } else if n == 1 {
        ONE.into()
    } else if n > 1 {
        format!(
            "{} + {}",
            ONE,
            (" + ".to_string() + ONE).repeat((n - 1) as usize)
        )
    } else if n == -1 {
        format!("-({})", ONE)
    } else {
        format!(
            "-({} + {})",
            ONE,
            (" + ".to_string() + ONE).repeat(((-n) - 1) as usize)
        )
    }
}

/// Encodes an arbitrary character into a JSFuck character
pub fn encode_character(c: char) -> String {
    // We first need to get the characters
    // c, o, n, s, t, r, u, f, o, m, C, h, a, d, e
    match c {
        'a' => "(![]+[])[+!+[]]".into(),
        _ => todo!(),
    }
}

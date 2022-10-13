/// Encodes zero. Works because `+` coerces `[]` into an integer type: `0`.
const ZERO: &str = "+[]";

/// Encodes one. Works because `!` coerces `[]` into a boolean type (`false`),
/// and then the next `!` coerces it into `true`. The final `+` coerces it
/// into an integer type, which is `1`, as the value is `true`.
const ONE: &str = " +!![] ";

/// Encodes true. Works becuase `+[]` is zero, and `!` coerces it into `true`
/// (as `0` is `false`, so `!0` is `true`).
const TRUE: &str = "!+[]";

/// Encodes false. Works because `!` coerces `[]` into a boolean type (`false`).
const FALSE: &str = "![]";

/// Encodes any number into a JSFuck number
pub fn encode_number(n: isize) -> String {
    if n == 0 {
        ZERO.into() // just return zero
    } else if n == 1 {
        ONE.into() // just return one
    } else {
        format!(
            "{} + {}",
            ONE,
            (" + ".to_string() + ONE).repeat((n - 1) as usize)
        )
    }
}

/// Encodes an arbitrary character into a JSFuck character
pub fn encode_character(c: char) -> String {
    // We first need to get the characters to make `
    // c, o, t, r, u, m, C, h, d
    match c {
        'a' => format!("(![]+[])[{}]", encode_number(1)),
        'e' => format!("(![]+[])[{}]", encode_number(4)),
        'f' => format!("(![]+[])[{}]", encode_number(0)),
        'i' => format!(
            "([]+ +(+!+[]+(!+[]+[])[!+[]+!+[]+!+[]]+[+!+[]]+[+[]]+[+[]]+[+[]]))[{}]",
            encode_number(3)
        ),
        'l' => format!("(![]+[])[{}]", encode_number(2)),
        'n' => format!(
            "([]+ +(+!+[]+(!+[]+[])[!+[]+!+[]+!+[]]+[+!+[]]+[+[]]+[+[]]+[+[]]))[{}]",
            encode_number(1)
        ),
        'r' => format!("(!+[]+[])[{}]", encode_number(1)),
        's' => format!("(![]+[])[{}]", encode_number(3)),
        't' => format!("(!+[]+[])[{}]", encode_number(0)),
        _ => todo!(),
    }
}

/// Encodes an arbitrary string into JSFuck characters.
/// Note: returns js string type.
pub fn encode_string(s: &str) -> String {
    s.chars() //gets the characters
        .map(encode_character) // maps them into jsfuck chars
        .collect::<Vec<String>>() // collects them for the next part
        .join("+") // joins and concatenates them as a string
}

pub fn verse(n: u32) -> String {
    format!(
        "{}\n{}\n",
        match n {
            0 => "No more bottles of beer on the wall, no more bottles of beer.".to_string(),
            n @ 1 => format!("{n} bottle of beer on the wall, {n} bottle of beer."),
            n => format!("{n} bottles of beer on the wall, {n} bottles of beer."),
        },
        match n.checked_sub(1) {
            Some(0) =>
                "Take it down and pass it around, no more bottles of beer on the wall.".to_string(),
            Some(x) if x == 1 =>
                format!("Take one down and pass it around, {x} bottle of beer on the wall."),
            Some(x) =>
                format!("Take one down and pass it around, {x} bottles of beer on the wall."),
            None =>
                "Go to the store and buy some more, 99 bottles of beer on the wall.".to_string(),
        }
    )
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .map(|n| dbg!(verse(n)))
        .rev()
        .collect::<Vec<String>>()
        .join("\n")
}

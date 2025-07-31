pub fn encode(source: &str) -> String {
    match source.chars().next() {
        None => String::new(),
        Some(head) => {
            let count = source.chars().take_while(|c| head.eq(c)).count();
            format!(
                "{}{head}{}"
                , if count == 1 { String::new() } else { count.to_string() }
                , encode(&source[count..])
            )
        }
    }
}

pub fn decode(source: &str) -> String {
    use std::iter::repeat;
    match source.chars().next_back() {
        None => String::new(),
        Some(tail) => {
            let count: String = source.chars().rev().skip(1).take_while(|c| c.is_numeric())
                .collect::<String>().chars().rev().collect();
            format!(
                "{}{}"
                , decode(&source[..(source.len() - count.len() - 1)])
                , repeat(tail).take(count.parse::<usize>().unwrap_or(1)).collect::<String>() 
            )
        }
    }
}

pub fn reply(message: &str) -> &str {
    const ANSWERS: [&str; 5] = [
        "Whatever.",
        "Sure.",
        "Whoa, chill out!",
        "Calm down, I know what I'm doing!",
        "Fine. Be that way!",
    ];
    const RULES: [fn(&str) -> bool; 3] = [
        |m| m.ends_with('?'), // Is a question.
        |m| !m.is_empty() // If not empty..
                && m.chars().any(|c| c.is_alphabetic()) // ..and contains any letter..
                && m.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase()), // ..and all are uppercase.
        |m| m.is_empty() // Is silence.
    ];
    let message = message.trim();
    let answer = RULES.iter().enumerate().fold(0usize, |a, (i, r)| a | (r(message) as usize) << i); // Set a bitmap.
    ANSWERS[answer]
}

pub fn brackets_are_balanced(string: &str) -> bool {
    const BRACKETS: [(char, char); 3] = [('[', ']'), ('(', ')'), ('{', '}')];
    let mut sb_stack: Vec<char> = Vec::new();
    
    for c in string.chars() {
        if BRACKETS.iter().any(|b| c.eq(&b.0)) {
            sb_stack.push(c)
        } else if let Some(b) = BRACKETS.iter().find(|b| c.eq(&b.1)) {
            match sb_stack.pop() {
                Some(sb) if sb.eq(&b.0) => {},
                _ => return false // exit forcibly
            }
        } else {}
    }

    string.is_empty() // No brackets or..
        // ..starting and closing brackets are balanced.
        || sb_stack.capacity() != 0 && sb_stack.is_empty()
}

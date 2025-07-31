pub fn raindrops(n: u32) -> String {
    match [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .iter()
        .flat_map(|(f, p)| if n % f == 0 { *p } else { "" }.chars())
        .collect::<String>()
    {
        s if s.is_empty() => n.to_string(),
        s => s,
    }
}

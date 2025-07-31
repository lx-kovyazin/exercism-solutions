use std::collections::HashMap;

fn counter(key: char, val: usize, map: &mut HashMap<char, usize>) {
    use std::ops::AddAssign;
    map.entry(key)
        .and_modify(|counter| counter.add_assign(val))
        .or_insert(val);
}

fn frequency_seq(input: &[&str]) -> HashMap<char, usize> {
    let mut map: HashMap<char, usize> = HashMap::new();
    for s in input {
        for c in s.chars().filter(|c| c.is_alphabetic()) {
            if let Some(c) = c.to_lowercase().next() {
                counter(c, 1, &mut map);
            }
        }
    }
    map
}

fn frequency_par(input: &'static [&str], worker_count: usize) -> HashMap<char, usize> {
    use std::thread::spawn;
    input
        .chunks(input.len() / worker_count)
        .map(|chunk| spawn(move || frequency_seq(chunk)))
        .collect::<Vec<_>>()
        .into_iter()
        .map(|thread| thread.join().unwrap())
        .fold(HashMap::new(), |mut map, res| {
            res.into_iter().for_each(|(k, v)| counter(k, v, &mut map));
            map
        })
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    match input.len().min(worker_count) {
        0 | 1 => frequency_seq(input),
        worker_count => {
            use std::mem::transmute;
            let input: &'static [&str] = unsafe { transmute(input) };
            frequency_par(input, worker_count)
        }
    }
}

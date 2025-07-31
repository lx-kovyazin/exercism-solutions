use std::{collections::HashMap, ops::AddAssign, thread};

fn chunk_size(len: usize, count: usize) -> Option<usize> {
    match len {
        0 => None,
        l => Some(l / count + if l % count != 0 { 1 } else { 0 }),
    }
}

fn clear(input: &[&str]) -> Vec<char> {
    input
        .iter()
        .flat_map(|s| s.chars())
        .filter(|c| c.is_alphabetic())
        .flat_map(|c| c.to_lowercase())
        .collect()
}

fn clear_parallel(input: &[&str], worker_count: usize) -> Vec<char> {
    match chunk_size(input.len(), worker_count) {
        None => Vec::new(),
        Some(size) => thread::scope(|scope| {
            let threads: Vec<_> = input
                .chunks(size)
                .map(|chunk| scope.spawn(|| clear(chunk)))
                .collect();
            threads
                .into_iter()
                .map(|thread| thread.join())
                .filter_map(|r| r.ok())
                .flatten()
                .collect()
        }),
    }
}

fn counter(key: char, val: usize, map: &mut HashMap<char, usize>) {
    map.entry(key).and_modify(|counter| counter.add_assign(val)).or_insert(val);
}

fn merge_maps(maps: Vec<HashMap<char, usize>>) -> HashMap<char, usize> {
    let mut res: HashMap<char, usize> = HashMap::new();
    maps.iter().fold(&mut res, |lhs, rhs| {
        rhs.into_iter().for_each(|(&k, &v)| counter(k, v, lhs));
        lhs
    });
    res
}

fn frequency_parallel(line: Vec<char>, worker_count: usize) -> HashMap<char, usize> {
    match chunk_size(line.len(), worker_count) {
        None => HashMap::new(),
        Some(size) => {
            let maps: Vec<_> = thread::scope(|scope| {
                let threads: Vec<_> = line
                    .chunks(size)
                    .map(|chunk| {
                        scope.spawn(move || {
                            let mut map: HashMap<char, usize> = HashMap::new();
                            chunk.iter().for_each(|&c| counter(c, 1, &mut map));
                            map
                        })
                    })
                    .collect();
                threads
                    .into_iter()
                    .map(|thread| thread.join())
                    .filter_map(|r| r.ok())
                    .collect()
            });
            merge_maps(maps)
        }
    }
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    frequency_parallel(clear_parallel(input, worker_count), worker_count)
}

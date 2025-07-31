use itertools::Itertools;

fn saturating_add_signed(lhs: usize, rhs: isize) -> usize {
    (if rhs >= 0 { usize::saturating_add } else { usize::saturating_sub })(lhs, rhs.unsigned_abs())
}

fn offset_ij(src: &(usize, usize), offset: &(isize, isize)) -> (usize, usize) {
    (saturating_add_signed(src.0, offset.0), saturating_add_signed(src.1, offset.1))
}

fn neighbors<'a>(src: &'a (usize, usize), dim: &'a (usize, usize))
    -> impl Iterator<Item = (usize, usize)> + 'a {
    (-1isize..=1).cartesian_product(-1isize..=1)
                 .map(|o| offset_ij(src, &o))
                 .unique()
                 .filter(|n| n.ne(src) && n.0 < dim.0 && n.1 < dim.1)
}

fn increment_count(elem: &mut u8) {
    use std::ops::AddAssign;
    match elem {
        b'*' => return,
        b' ' => elem.add_assign(0x10), // offset to zero (0x30)
        _    => (),
    }
    elem.add_assign(1)
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut out = minefield.iter().map(|row| row.as_bytes().to_vec()).collect_vec();
    let n = out.len();
    let m = match out.first() {
        None => return Vec::new(),
        Some(f) => f.len()
    };
    for i in 0..n {
        for j in 0..m {
            if let b'*' = out[i][j] {
                neighbors(&(i, j), &(n, m)).for_each(|(ni, nj)| increment_count(&mut out[ni][nj]));
            }
        }
    }
    unsafe { std::mem::transmute(out) }
}

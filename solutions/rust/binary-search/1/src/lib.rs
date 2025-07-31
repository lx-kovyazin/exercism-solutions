pub fn find<A, T>(array: A, key: T) -> Option<usize>
where
    A: AsRef<[T]>,
    T: Ord,
{
    let array = array.as_ref();
    match array.len() {
        0 => None,
        l => {
            let mid = l / 2;
            match &array[mid] {
                guess if key.lt(guess) => find(&array[0..mid], key),
                guess if key.gt(guess) => {
                    let left = mid + 1;
                    match find(&array[left..l], key) {
                        None => None,
                        Some(rhs) => Some(left + rhs),
                    }
                }
                _ => Some(mid),
            }
        }
    }
}

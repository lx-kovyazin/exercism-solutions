#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    return if first_list.eq(second_list) {
        Comparison::Equal
    } else if lhs_is_sublist_rhs(first_list, second_list) {
        Comparison::Sublist
    } else if lhs_is_sublist_rhs(second_list, first_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    };

    fn lhs_is_sublist_rhs<T: PartialEq>(lhs: &[T], rhs: &[T]) -> bool {
        lhs.is_empty() || rhs.windows(lhs.len()).any(|w| w.eq(lhs))
    }
}

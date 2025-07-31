use std::iter::Sum;
use itertools::Itertools;

pub struct Triangle<T>(Vec<Vec<T>>);

impl<T> Triangle<T>
where
    T: From<i32> + Copy + PartialOrd + Sum
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let side_combinations = sides.into_iter().combinations(2).collect_vec();
        (
            sides.into_iter().all(|side| side > 0.into())
         && sides.into_iter().rev().zip(
                    side_combinations.iter().map(|comb| comb.clone().into_iter().sum::<T>())
                 )
                .all(|(side, sum_other_sides)| side <= sum_other_sides)
        )
        .then_some(Self(side_combinations))
        .or(None)
    }

    pub fn is_equilateral(&self) -> bool {
        self.0.iter().all(|comb| comb.iter().all_equal())
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        self.0.iter().any(|comb| comb.iter().all_equal())
    }
}

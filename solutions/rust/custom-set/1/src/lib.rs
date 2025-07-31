use std::{collections::HashSet, hash::Hash};

#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T>(HashSet<T>)
where
    T: Hash + Eq + Clone;

impl<T> CustomSet<T>
where
    T: Hash + Eq + Clone,
{
    pub fn new(input: &[T]) -> Self {
        Self(HashSet::from_iter(input.iter().cloned()))
    }

    pub fn contains(&self, element: &T) -> bool {
        self.0.contains(element)
    }

    pub fn add(&mut self, element: T) {
        self.0.insert(element);
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.0.is_subset(&other.0)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.0.is_disjoint(&other.0)
    }

    pub fn intersection(&self, other: &Self) -> Self {
        Self(self.0.intersection(&other.0).cloned().collect())
    }

    pub fn difference(&self, other: &Self) -> Self {
        Self(self.0.difference(&other.0).cloned().collect())
    }

    pub fn union(&self, other: &Self) -> Self {
        Self(self.0.union(&other.0).cloned().collect())
    }
}

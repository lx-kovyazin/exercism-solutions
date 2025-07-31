use std::ops::Rem;

pub struct Matcher<T>(Box<dyn Fn(T) -> Option<String>>);

impl<T> Matcher<T> {
    pub fn new<F, S>(matcher: F, subs: S) -> Matcher<T>
    where
        F: Fn(T) -> bool + 'static,
        S: ToString + 'static,
    {
        Self(Box::new(move |i| matcher(i).then(|| subs.to_string())))
    }
}

pub struct Fizzy<T>(Vec<Matcher<T>>);

impl<T> Fizzy<T>
where
    T: Clone + Copy + ToString + Rem<T, Output = T>,
{
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.0.push(matcher);
        self
    }

    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = T>,
    {
        iter.map(
            move |i| match self.0.iter().filter_map(|m| m.0(i)).collect::<String>() {
                e if e.is_empty() => i.to_string(),
                s => s,
            },
        )
    }
}

pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: Clone + Copy + PartialEq + ToString + From<u8> + Rem<T, Output = T>,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|n| n % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|n| n % 5.into() == 0.into(), "buzz"))
}

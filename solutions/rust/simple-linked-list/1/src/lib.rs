use std::iter::FromIterator;

struct Node<T> {
    value: T,
    prev: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Box::new(Node {
            value: element,
            prev: self.head.take(),
        }));
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let old_head = *self.head.take()?;
        self.head = old_head.prev;
        self.len -= 1;
        Some(old_head.value)
    }

    pub fn peek(&self) -> Option<&T> {
        Some(&self.head.as_deref()?.value)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut old = self;
        let mut new = Self::new();
        while !old.is_empty() {
            new.push(old.pop().unwrap());
        }
        new
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().fold(SimpleLinkedList::new(), |mut l, i| {
            l.push(i);
            l
        })
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut linked_list = linked_list.rev();
        let mut res: Vec<T> = Vec::with_capacity(linked_list.len());
        while !linked_list.is_empty() {
            res.push(linked_list.pop().unwrap())
        }
        res
    }
}

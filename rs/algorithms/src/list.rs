use std::fmt::Debug;
use std::iter::FromIterator;

struct Node<T> {
    value: T,
    next: Box<List<T>>,
}

pub struct List<T>(Option<Node<T>>);

impl<T> List<T> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        List(None)
    }

    #[allow(dead_code)]
    pub fn with(value: T) -> Self {
        List(Some(Node {
            value,
            next: Box::new(List(None)),
        }))
    }

    #[allow(dead_code)]
    pub fn push_mut(&mut self, value: T) {
        *self = List(Some(Node {
            value,
            next: Box::new(List(self.0.take())),
        }))
    }

    #[allow(dead_code)]
    pub fn push(self, value: T) -> Self {
        List(Some(Node {
            value,
            next: Box::new(List(self.0)),
        }))
    }

    #[allow(dead_code)]
    pub fn pop_mut(&mut self) -> Option<T> {
        self.0.take().map(|Node { value, next }| {
            *self = *next;
            value
        })
    }

    #[allow(dead_code)]
    pub fn pop(self) -> (Self, Option<T>) {
        match self.0 {
            Some(Node { value, next }) => (*next, Some(value)),
            None => (self, None),
        }
    }
}

impl<T> Iterator for List<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_mut()
    }
}

impl<T> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        iter.into_iter().fold(List(None), |list, value| {
            list.push(value)
        })
    }
}

struct Iter<'a, T>(Option<&'a Node<T>>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.map(|Node { value, next }| {
            self.0 = next.as_ref().0.as_ref();
            value
        })
    }
}

impl<T: Debug> Debug for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 {
            Some(ref n) => {
                write!(f, "{:?}; {:?}", n.value, n.next)
            }
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let list = [1, 2, 3].iter().cloned().collect::<List<i32>>();

        let list = list.push(10);
        let list = list.push(20);
        let (list, popped_value) = list.pop();

        assert_eq!(list.collect::<Vec<i32>>(), vec![10, 3, 2, 1]);
        assert_eq!(popped_value, Some(20));
    }
}
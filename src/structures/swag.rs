use std::mem::swap;

pub struct Swag<T, F> {
    f: F,
    head: Vec<T>,
    f_head: Vec<T>,
    tail: Vec<T>,
    f_tail: Vec<T>,
}

impl<T: Clone, F: Fn(&T, &T) -> T> Swag<T, F> {
    pub fn new(f: F) -> Self {
        Self {
            f,
            head: Vec::new(),
            f_head: Vec::new(),
            tail: Vec::new(),
            f_tail: Vec::new(),
        }
    }

    pub fn push(&mut self, x: T) {
        self.f_tail.push(if let Some(y) = self.f_tail.last() {
            (self.f)(y, &x)
        } else {
            x.clone()
        });
        self.tail.push(x);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_empty() {
            swap(&mut self.head, &mut self.tail);
            swap(&mut self.f_head, &mut self.f_tail);
        }
        self.f_head.pop();
        self.head.pop()
    }

    pub fn fold(&self) -> Option<T> {
        match (self.f_head.last(), self.f_tail.last()) {
            (Some(x), Some(y)) => Some((self.f)(x, y)),
            (Some(x), None) => Some(x.clone()),
            (None, Some(y)) => Some(y.clone()),
            _ => None,
        }
    }

    pub fn len(&self) -> usize {
        self.head.len() + self.tail.len()
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_empty() && self.tail.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sliding_minimum_test() {
        let mut swag = Swag::new(|x, y| i32::min(*x, *y));
        assert_eq!(swag.fold(), None);
        swag.push(5);
        assert_eq!(swag.fold(), Some(5));
        swag.push(3);
        assert_eq!(swag.fold(), Some(3));
        assert_eq!(swag.len(), 2);
        assert_eq!(swag.pop(), Some(3));
        assert_eq!(swag.fold(), Some(5));
        swag.push(4);
        assert_eq!(swag.fold(), Some(4));
        assert_eq!(swag.pop(), Some(5));
        assert_eq!(swag.fold(), Some(4));
        assert_eq!(swag.pop(), Some(4));
        assert_eq!(swag.fold(), None);
        assert_eq!(swag.pop(), None);
    }
}

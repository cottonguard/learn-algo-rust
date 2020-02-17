use std::mem::swap;

#[derive(Clone, Default)]
pub struct SkewHeap<T> {
    inner: Option<Box<Inner<T>>>,
}

#[derive(Clone)]
struct Inner<T> {
    value: T,
    left: SkewHeap<T>,
    right: SkewHeap<T>,
}

impl<T: Ord> SkewHeap<T> {
    pub fn new() -> Self {
        Self { inner: None }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_none()
    }

    pub fn meld(&mut self, mut other: Self) {
        if self.is_empty() {
            *self = other;
        } else if let (Some(a), Some(b)) = (self.inner.as_mut(), other.inner.as_mut()) {
            if a.value < b.value {
                swap(a, b);
            }
            a.right.meld(other);
            swap(&mut a.left, &mut a.right);
        }
    }

    pub fn push(&mut self, value: T) {
        self.meld(Self {
            inner: Some(Box::new(Inner {
                value,
                left: Self::new(),
                right: Self::new(),
            })),
        });
    }

    pub fn top(&self) -> Option<&T> {
        self.inner.as_ref().map(|i| &i.value)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.inner.take().map(|inner| {
            let Inner {
                mut left,
                right,
                value,
            } = *inner;
            left.meld(right);
            *self = left;
            value
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut h1 = SkewHeap::new();
        assert!(h1.is_empty());
        h1.push(5);
        assert!(!h1.is_empty());
        h1.push(2);
        h1.push(3);
        let mut h2 = SkewHeap::new();
        h2.push(4);
        h2.push(1);
        h2.push(6);
        h1.meld(h2);
        for i in (1..=6).rev() {
            assert_eq!(h1.top(), Some(&i));
            assert_eq!(h1.pop(), Some(i));
        }
        assert_eq!(h1.pop(), None);
    }
}

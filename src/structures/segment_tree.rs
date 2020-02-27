pub struct SegmentTree<T, Z, F> {
    zero: Z,
    f: F,
    a: Vec<T>,
}

impl<T, Z: Fn() -> T, F: Fn(&T, &T) -> T> SegmentTree<T, Z, F> {
    pub fn new(n: usize, zero: Z, f: F) -> Self {
        let n = n.next_power_of_two() << 1;
        Self {
            a: std::iter::repeat_with(|| zero()).take(n).collect(),
            zero,
            f,
        }
    }

    pub fn len(&self) -> usize {
        self.a.len() >> 1
    }

    pub fn set(&mut self, i: usize, value: T) {
        let mut i = self.len() + i;
        self.a[i] = value;
        while (i >> 1) > 0 {
            i >>= 1;
            self.a[i] = (self.f)(&self.a[i << 1], &self.a[(i << 1) + 1]);
        }
    }

    pub fn update(&mut self, i: usize, value: &T) -> &T {
        let j = self.len() + i;
        self.set(i, (self.f)(&value, &self.a[j]));
        &self.a[j]
    }

    // [l, r)
    pub fn fold(&self, l: usize, r: usize) -> T {
        let mut l = self.len() + l;
        let mut r = self.len() + r;
        let mut x = (self.zero)();
        let mut y = (self.zero)();
        while l != r {
            if l & 1 == 1 {
                x = (self.f)(&x, &self.a[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                y = (self.f)(&self.a[r], &y);
            }
            l >>= 1;
            r >>= 1;
        }
        (self.f)(&x, &y)
    }
}

impl<T, Z: Fn() -> T, F: Fn(&T, &T) -> T> std::ops::Index<usize> for SegmentTree<T, Z, F> {
    type Output = T;
    fn index(&self, i: usize) -> &T {
        &self.a[self.len() + i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segment_tree_test() {
        let mut st = SegmentTree::new(6, i32::max_value, |x, y| i32::min(*x, *y));
        assert_eq!(st.len(), 8);
        st.set(0, 2);
        st.set(2, 7);
        st.set(5, 5);
        assert_eq!(st.fold(1, 6), 5);
        st.set(3, 4);
        assert_eq!(st.fold(2, 6), 4);
        assert_eq!(st[3], 4);
        st.set(3, 6);
        assert_eq!(st.fold(1, 6), 5);
        assert_eq!(st.fold(0, 6), 2);
    }
}

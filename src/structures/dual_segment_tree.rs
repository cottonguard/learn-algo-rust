#[derive(Clone, Debug)]
pub struct DualSegmentTree<T> {
    ops: Vec<T>,
}

impl<T: Monoid> DualSegmentTree<T> {
    pub fn new(n: usize) -> Self {
        use std::iter::repeat_with;

        let m = 2 * n.next_power_of_two();
        Self {
            ops: repeat_with(T::zero).take(m).collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.ops.len() / 2
    }

    pub fn apply(&mut self, l: usize, r: usize, op: &T) {
        self.apply_rec(0, 0, self.len(), l, r, op);
    }

    fn apply_rec(&mut self, k: usize, il: usize, ir: usize, l: usize, r: usize, op: &T) {
        if l <= il && ir <= r {
            self.ops[k] = op.plus(&self.ops[k]);
        } else if l < ir && il < r {
            let l_cld = 2 * k + 1;
            let r_cld = 2 * k + 2;
            let im = il + (ir - il) / 2;
            self.ops[l_cld] = self.ops[k].plus(&self.ops[l_cld]);
            self.ops[r_cld] = self.ops[k].plus(&self.ops[r_cld]);
            self.ops[k] = T::zero();
            self.apply_rec(l_cld, il, im, l, r, op);
            self.apply_rec(r_cld, im, ir, l, r, op);
        }
    }

    pub fn get(&self, i: usize) -> T {
        let mut i = i + self.len() - 1;
        let mut op = self.ops[i].plus(&T::zero());
        loop {
            i >>= 1;
            op = self.ops[i].plus(&op);
            if i == 0 {
                break;
            }
        }
        op
    }
}

pub trait Monoid {
    fn zero() -> Self;
    fn plus(&self, other: &Self) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(PartialEq, Clone, Debug)]
    enum Update<T: Clone> {
        Updated(T),
        None,
    }
    impl<T: Clone> Monoid for Update<T> {
        fn zero() -> Self {
            Self::None
        }
        fn plus(&self, other: &Self) -> Self {
            match self {
                Self::Updated(_) => self.clone(),
                Self::None => other.clone(),
            }
        }
    }

    #[test]
    fn range_update_query_test() {
        let mut dst = DualSegmentTree::new(8);
        dst.apply(0, 6, &Update::Updated(10));
        dst.apply(1, 3, &Update::Updated(20));
        assert_eq!(dst.get(0), Update::Updated(10));
        assert_eq!(dst.get(2), Update::Updated(20));
        dst.apply(2, 4, &Update::Updated(30));
        assert_eq!(dst.get(2), Update::Updated(30));
    }
}

// totally wip

pub struct SplayTreeMap<K, V> {
    len: usize,
    root: Node<K, V>,
}
enum Node<K, V> {
    Node(Box<Inner<K, V>>),
    Nil,
}
struct Inner<K, V> {
    key: K,
    value: V,
    left: Node<K, V>,
    right: Node<K, V>,
}
impl<K: Ord, V> SplayTreeMap<K, V> {
    pub fn new() -> Self {
        SplayTreeMap { len: 0, root: Nil }
    }
    fn splay(&mut self, key: &K) {
        self.root = self.root.take().splay(key);
    }
    pub fn get(&mut self, key: &K) -> Option<&V> {
        self.get_mut(key).map(|value| &*value)
    }
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.splay(key);
        if let Node(root) = &mut self.root {
            Some(&mut root.value)
        } else {
            None
        }
    }
    pub fn insert(&mut self, key: K, value: V) {
        let (v, inserted) = self.root.take().insert(key, value);
        self.root = v;
        self.len += inserted as usize;
    }
    pub fn remove(&mut self, key: &K) {
        let (v, removed) = self.root.take().remove(key);
        self.root = v;
        self.len -= removed as usize;
    }
}
use Node::*;
impl<K: Ord, V> Node<K, V> {
    fn node(key: K, value: V, left: Self, right: Self) -> Self {
        Node(Box::new(Inner {
            key,
            value,
            left,
            right,
        }))
    }
    fn splay(self, x: &K) -> Self {
        use std::cmp::Ordering::*;
        if let Node(mut v) = self {
            match x.cmp(&v.key) {
                Equal => Node(v),
                Less => {
                    if let Node(mut l) = v.left.take() {
                        match x.cmp(&l.key) {
                            Equal => Node(v).rot_r(Node(l)),
                            Less => {
                                let ll = l.left.take().splay(x);
                                Node(v).rot_r(Node(l)).rot_r(ll)
                            }
                            Greater => {
                                let lr = l.right.take().splay(x);
                                Node(v).rot_r(Node(l).rot_l(lr))
                            }
                        }
                    } else {
                        Node(v)
                    }
                }
                Greater => {
                    if let Node(mut r) = v.right.take() {
                        match x.cmp(&r.key) {
                            Equal => Node(v).rot_l(Node(r)),
                            Less => {
                                let rl = r.left.take().splay(x);
                                Node(v).rot_l(Node(r).rot_r(rl))
                            }
                            Greater => {
                                let rr = r.right.take().splay(x);
                                Node(v).rot_l(Node(r)).rot_l(rr)
                            }
                        }
                    } else {
                        Node(v)
                    }
                }
            }
        } else {
            self
        }
    }
    fn replace(&mut self, inner: Box<Inner<K, V>>) -> Self {
        std::mem::replace(self, Node(inner))
    }
    fn take(&mut self) -> Self {
        std::mem::replace(self, Nil)
    }
    fn rot_r(self, l: Self) -> Self {
        match (self, l) {
            (Node(mut v), Node(mut l)) => {
                v.left = l.right.take();
                l.right.replace(v);
                Node(l)
            }
            (Node(v), Nil) => Node(v),
            (Nil, Node(l)) => Node(l),
            _ => Nil,
        }
    }
    fn rot_l(self, r: Self) -> Self {
        match (self, r) {
            (Node(mut v), Node(mut r)) => {
                v.right = r.left.take();
                r.left.replace(v);
                Node(r)
            }
            (Node(v), Nil) => Node(v),
            (Nil, Node(r)) => Node(r),
            _ => Nil,
        }
    }
    fn split(self, x: &K) -> (Self, Self) {
        if let Node(mut v) = self.splay(&x) {
            if x < &v.key {
                let l = v.left.take();
                (l, Node(v))
            } else {
                let r = v.right.take();
                (Node(v), r)
            }
        } else {
            (Nil, Nil)
        }
    }
    fn merge(self, r: Self) -> Self {
        match (self, r) {
            (l @ Node(_), Node(r)) => {
                let mut v = l.splay(&r.key);
                if let Node(v) = &mut v {
                    v.right.replace(r);
                }
                v
            }
            (l @ Node(_), Nil) => l,
            (Nil, r @ Node(_)) => r,
            _ => Nil,
        }
    }
    fn insert(mut self, key: K, value: V) -> (Self, bool) {
        self = self.splay(&key);
        if let Node(v) = &mut self {
            if key == v.key {
                v.value = value;
                return (self, false);
            }
        }
        let (l, r) = self.split(&key);
        (Self::node(key, value, l, r), true)
    }
    fn remove(mut self, x: &K) -> (Self, bool) {
        self = self.splay(x);
        if let Node(v) = self {
            (v.left.merge(v.right), true)
        } else {
            (self, false)
        }
    }
}

use std::fmt;
impl<K: fmt::Display, V> fmt::Display for SplayTreeMap<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.root.fmt(f)
    }
}
impl<K: fmt::Display, V> fmt::Display for Node<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Node(v) = self {
            write!(f, "(")?;
            v.left.fmt(f)?;
            write!(f, " {} ", v.key)?;
            v.right.fmt(f)?;
            write!(f, ")")?;
        } else {
            write!(f, ".")?;
        }
        Ok(())
    }
}
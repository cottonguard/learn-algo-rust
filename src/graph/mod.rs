pub struct Vertex<T> {
    pub value: T,
}

pub struct Edge<T> {
    pub dest: usize,
    pub value: T,
}

struct VertexNode<T> {
    inner: Vertex<T>,
    first: usize,
    last: usize,
}

struct EdgeNode<T> {
    inner: Edge<T>,
    next: usize,
}

pub struct Graph<T, U> {
    vers: Vec<VertexNode<T>>,
    edges: Vec<EdgeNode<U>>,
}

const NIL: usize = usize::max_value();

impl<T: Default, U> Graph<T, U> {
    pub fn new(n: usize) -> Self {
        Self {
            vers: std::iter::repeat_with(|| VertexNode {
                inner: Vertex {
                    value: T::default(),
                },
                first: NIL,
                last: NIL,
            })
            .take(n)
            .collect(),
            edges: Vec::new(),
        }
    }
}

impl<T, U> Graph<T, U> {
    pub fn add_edge(&mut self, u: usize, v: usize, value: U) -> &Edge<U> {
        let idx = self.edges.len();
        self.edges.push(EdgeNode {
            inner: Edge { dest: v, value },
            next: NIL,
        });
        if self.vers[u].first == NIL {
            self.vers[u].first = idx;
        } else {
            self.edges[self.vers[u].last].next = idx;
        }
        self.vers[u].last = idx;
        &self.edges.last().unwrap().inner
    }

    pub fn edges(&self, u: usize) -> Edges<T, U> {
        Edges {
            g: self,
            idx: self.vers[u].first,
        }
    }

    pub fn degree(&self, u: usize) -> usize {
        self.edges(u).count()
    }
}

pub struct Edges<'a, T, U> {
    g: &'a Graph<T, U>,
    idx: usize,
}

impl<'a, T, U> Iterator for Edges<'a, T, U> {
    type Item = &'a Edge<U>;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.g.edges.get(self.idx).map(|e| {
            self.idx = e.next;
            &e.inner
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut g: Graph<(), i32> = Graph::new(5);
        g.add_edge(3, 2, 20);
        g.add_edge(0, 1, 10);
        g.add_edge(1, 2, 20);
        g.add_edge(0, 2, 30);
        assert_eq!(g.degree(0), 2);
        assert_eq!(g.edges(0).fold(0, |w, e| w + e.value), 40);
    }
}
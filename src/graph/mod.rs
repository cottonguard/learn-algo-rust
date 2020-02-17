pub mod shortest_path;

#[derive(Clone)]
pub struct Edge<T> {
    pub dest: usize,
    pub value: T,
}

#[derive(Clone, Copy)]
struct VertexNode {
    first: usize,
    last: usize,
}

#[derive(Clone)]
struct EdgeNode<T> {
    inner: Edge<T>,
    next: usize,
}

#[derive(Clone)]
pub struct Graph<T> {
    vers: Vec<VertexNode>,
    edges: Vec<EdgeNode<T>>,
}

const NIL: usize = usize::max_value();

impl<T> Graph<T> {
    pub fn new(n: usize) -> Self {
        let mut g = Self::default();
        g.resize(n);
        g
    }

    pub fn size(&self) -> usize {
        self.vers.len()
    }

    pub fn resize(&mut self, n: usize) {
        self.vers.resize(
            n,
            VertexNode {
                first: NIL,
                last: NIL,
            },
        );
    }

    pub fn add_edge(&mut self, u: usize, v: usize, value: T) -> &Edge<T> {
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

    pub fn edges(&self, u: usize) -> Edges<T> {
        Edges {
            g: self,
            idx: self.vers[u].first,
        }
    }

    pub fn degree(&self, u: usize) -> usize {
        self.edges(u).count()
    }
}

impl<T> Default for Graph<T> {
    fn default() -> Self {
        Self {
            vers: Vec::new(),
            edges: Vec::new(),
        }
    }
}

pub struct Edges<'a, T> {
    g: &'a Graph<T>,
    idx: usize,
}

impl<'a, T> Iterator for Edges<'a, T> {
    type Item = &'a Edge<T>;
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
        let mut g: Graph<i32> = Graph::new(5);
        g.add_edge(3, 2, 20);
        g.add_edge(0, 1, 10);
        g.add_edge(1, 2, 20);
        g.add_edge(0, 2, 30);
        assert_eq!(g.degree(0), 2);
        assert_eq!(g.edges(0).fold(0, |w, e| w + e.value), 40);
    }
}

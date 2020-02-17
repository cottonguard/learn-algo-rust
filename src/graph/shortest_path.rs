use super::Graph;
use num::Num;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn dijkstra<T: Copy + Num + Ord>(g: Graph<T>, s: usize) -> Vec<Option<T>> {
    let mut dist: Vec<Option<T>> = std::iter::repeat_with(|| None).take(g.size()).collect();
    let mut pq = BinaryHeap::new();
    dist[s] = Some(T::zero());
    pq.push((Reverse(T::zero()), s));
    while let Some((Reverse(d), u)) = pq.pop() {
        if Some(d) > dist[u] {
            continue;
        }
        for e in g.edges(u) {
            let new_d = d + e.value;
            if dist[e.dest].map_or(true, |d| new_d < d) {
                dist[e.dest] = Some(new_d);
                pq.push((Reverse(new_d), e.dest));
            }
        }
    }
    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut g: Graph<i32> = Graph::new(6);
        g.add_edge(1, 2, 10);
        g.add_edge(2, 4, 40);
        g.add_edge(2, 3, 30);
        g.add_edge(1, 4, 70);
        g.add_edge(1, 3, 30);
        g.add_edge(3, 4, 10);
        g.add_edge(4, 1, 10);
        assert_eq!(
            dijkstra(g, 1),
            [None, Some(0), Some(10), Some(30), Some(40), None]
        );
    }
}

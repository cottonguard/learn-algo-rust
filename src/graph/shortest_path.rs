use super::Graph;
use num::Num;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn dijkstra<T, U: Copy + Num + Ord>(g: Graph<T, U>, s: usize) -> Vec<Option<U>> {
    let mut dist: Vec<Option<U>> = std::iter::repeat_with(|| None).take(g.size()).collect();
    let mut pq = BinaryHeap::new();
    dist[s] = Some(U::zero());
    pq.push((Reverse(U::zero()), s));
    while let Some((Reverse(d), u)) = pq.pop() {
        if Some(d) > dist[u] {
            continue;
        }
        for e in g.edges(u) {
            let new_d = d + e.value;
            if dist[e.dest].is_none() || Some(new_d) < dist[e.dest] {
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
        let mut g: Graph<(), i32> = Graph::new(6);
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

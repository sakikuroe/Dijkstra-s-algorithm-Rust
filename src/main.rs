use std::{cmp::Ordering, collections::BinaryHeap};

macro_rules! chmin {
    ($a: expr, $b: expr) => {
        $a > $b && {
            $a = $b;
            true
        }
    };
}

#[derive(Eq, PartialEq)]
pub struct Node<T> {
    pub id: usize,
    pub priority: T,
}

impl<T> Ord for Node<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority).reverse()
    }
}

impl<T> PartialOrd for Node<T>
where
    T: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Node<T>
where
    T: Ord,
{
    pub fn new(id: usize, priority: T) -> Self {
        Node { id, priority }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Edge<T> {
    pub src: usize,
    pub dst: usize,
    pub weight: T,
}

impl<T> Ord for Edge<T>
where
    T: Eq + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl<T> PartialOrd for Edge<T>
where
    T: Eq + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Edge<T> {
    pub fn new(src: usize, dst: usize, weight: T) -> Edge<T> {
        Edge { src, dst, weight }
    }
}

pub struct Graph<T> {
    pub size: usize,
    pub edges: Vec<Vec<Edge<T>>>,
}

impl<T> Graph<T>
where
    T: Clone + Copy,
{
    pub fn new(size: usize) -> Self {
        Graph {
            size,
            edges: vec![vec![]; size],
        }
    }

    pub fn add_edge(&mut self, src: usize, dst: usize, weight: T) {
        self.edges[src].push(Edge::new(src, dst, weight));
    }

    pub fn add_undirected_edge(&mut self, src: usize, dst: usize, weight: T) {
        self.add_edge(src, dst, weight);
        self.add_edge(dst, src, weight);
    }
}

pub struct Dijkstra {
    dist: Vec<Option<usize>>,
    prev: Vec<Option<usize>>,
}

impl Dijkstra {
    pub fn get_distance(&self, dst: usize) -> Option<usize> {
        self.dist[dst]
    }

    pub fn get_path(&self, dst: usize) -> Option<Vec<usize>> {
        if self.dist[dst] == None {
            return None;
        }

        let mut id = dst;
        let mut res = vec![id];
        while let Some(prev_node) = self.prev[id] {
            id = prev_node;
            res.push(id);
        }
        res.reverse();
        return Some(res);
    }
}

impl Graph<usize> {
    fn dijkstra(&self, start: usize) -> Dijkstra {
        let mut dist = vec![std::usize::MAX; self.size];
        let mut prev = vec![None; self.size];
        dist[start] = 0;
        let mut que = BinaryHeap::new();
        que.push(Node::new(start, 0));

        while let Some(Node { id, priority }) = que.pop() {
            if priority > dist[id] {
                continue;
            }
            for &Edge { src, dst, weight } in &self.edges[id] {
                if chmin!(dist[dst], dist[src] + weight) {
                    prev[dst] = Some(src);
                    que.push(Node::new(dst, dist[dst]));
                }
            }
        }

        Dijkstra {
            dist: dist
                .into_iter()
                .map(|d| if d < std::usize::MAX { Some(d) } else { None })
                .collect(),
            prev,
        }
    }
}

fn main() {
    let mut g = Graph::new(10);
    g.add_edge(0, 1, 5 as usize);
    g.add_edge(0, 2, 3 as usize);
    g.add_edge(0, 3, 2 as usize);
    g.add_edge(0, 5, 2 as usize);
    g.add_edge(1, 3, 1 as usize);
    g.add_edge(2, 4, 1 as usize);
    g.add_edge(3, 4, 1 as usize);
    g.add_edge(4, 5, 3 as usize);

    let result = g.dijkstra(0);
    for i in 0..10 {
        print!("{}, {{Distance: {:?}, ", i, result.get_distance(i));

        print!("Path: ");
        if let Some(path) = result.get_path(i) {
            for j in 0..path.len() - 1 {
                print!("{} -> ", path[j]);
            }
            print!("{}", path[path.len() - 1]);
        } else {
            print!("The path does not exist.");
        }
        println!("}}");
    }
    // 0, {Distance: Some(0), Path: 0}
    // 1, {Distance: Some(5), Path: 0 -> 1}
    // 2, {Distance: Some(3), Path: 0 -> 2}
    // 3, {Distance: Some(2), Path: 0 -> 3}
    // 4, {Distance: Some(3), Path: 0 -> 3 -> 4}
    // 5, {Distance: Some(2), Path: 0 -> 5}
    // 6, {Distance: None, Path: The path does not exist.}
    // 7, {Distance: None, Path: The path does not exist.}
    // 8, {Distance: None, Path: The path does not exist.}
    // 9, {Distance: None, Path: The path does not exist.}
}
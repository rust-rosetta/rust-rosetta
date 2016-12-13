use std::collections::{HashMap, BinaryHeap, VecDeque};
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::iter::repeat;
use std::cmp::Ordering;
use std::usize;

type Node = usize;
type Cost = usize;
type Edge = (Node, Node);

/// The `DistPair` struct is for the priority queue.
#[derive(Eq, PartialEq, PartialOrd)]
struct DistPair(Node, Cost);
impl Ord for DistPair {
    fn cmp(&self, other: &DistPair) -> Ordering {
        let DistPair(_, dist_a) = *self;
        let DistPair(_, dist_b) = *other;
        dist_b.cmp(&dist_a) //Intentionally reversed
    }
}

/// Graph structure, represented as an adjacency List.
struct Graph<'a> {
    vertices: Vec<&'a str>,
    adj_list: Vec<Vec<Node>>,
    costs: HashMap<Edge, Cost>,
}

impl<'a> Graph<'a> {
    fn new() -> Graph<'a> {
        let vertices: Vec<&str> = Vec::new();
        let adj_list: Vec<Vec<Node>> = Vec::new();
        let costs: HashMap<Edge, Cost> = HashMap::new();

        Graph {
            vertices: vertices,
            adj_list: adj_list,
            costs: costs,
        }
    }

    /// Returns the index of the vertex, or `None` if vertex not found.
    fn vertex_index(&self, vertex: &str) -> Option<Node> {
        for (idx, &v) in self.vertices.iter().enumerate() {
            if v == vertex {
                return Some(idx);
            }
        }
        None
    }

    /// Returns the index of the vertex. If vertex is not found, inserts the vertex.
    fn get_or_insert_vertex(&mut self, vertex: &'a str) -> Node {
        self.vertex_index(vertex)
            .unwrap_or_else(|| {
                self.adj_list.push(Vec::new());
                self.vertices.push(vertex);
                self.vertices.len() - 1
            })
    }

    /// Adds the given edge to the graph.
    fn add_edge(&mut self, from: &'a str, to: &'a str, cost: usize) {
        let from_idx = self.get_or_insert_vertex(from);
        let to_idx = self.get_or_insert_vertex(to);

        match self.costs.entry((from_idx, to_idx)) {
            Vacant(entry) => {
                self.adj_list[from_idx].push(to_idx);
                entry.insert(cost);
            }
            Occupied(entry) => {
                *entry.into_mut() = cost;
            }
        };
    }

    /// Implements Dijkstra's algorithm. This uses a priority queue to determine which vertex to
    /// visit first. Terminates on discovering the target vertex.
    ///
    /// Returns vector of vertices representing the path, or an empty vector if there's no path, or
    /// if the source or target is not in the graph.
    fn dijkstra(&'a self, source: &str, target: &str) -> Vec<&str> {
        let num_vert = self.vertices.len();
        let mut dist: Vec<usize> = repeat(usize::MAX)
            .take(num_vert)
            .collect(); //Close enough to infinity
        let mut prev: HashMap<Node, Node> = HashMap::new();
        let mut queue: BinaryHeap<DistPair> = BinaryHeap::new();

        let source_idx = match self.vertex_index(source) {
            Some(idx) => idx,
            None => return Vec::new(), // Source not in graph, return empty path.
        };

        let target_idx = match self.vertex_index(target) {
            Some(idx) => idx,
            None => return Vec::new(), // Target not in graph, return empty path.
        };

        dist[source_idx] = 0;
        queue.push(DistPair(source_idx, dist[source_idx]));

        loop {
            match queue.pop() {
                None => break,
                Some(DistPair(u, dist_u)) => {
                    for &v in &(self.adj_list[u]) {
                        let cost_uv = match self.costs.get(&(u, v)) {
                            Some(&x) => x,
                            None => usize::MAX,
                        };
                        let alt = dist_u + cost_uv;
                        if alt < dist[v] {
                            match prev.entry(v) {
                                Vacant(entry) => {
                                    entry.insert(u);
                                }
                                Occupied(entry) => {
                                    *entry.into_mut() = u;
                                }
                            }
                            dist[v] = alt;
                            queue.push(DistPair(v, dist[v]));
                        }
                        if v == target_idx {
                            break;
                        }
                    }
                }
            };
        }

        let mut temp_path: VecDeque<&str> = VecDeque::new();
        let mut curr = target_idx;
        temp_path.push_front(self.vertices[curr]);
        loop {
            match prev.get(&curr) {
                Some(&parent) => {
                    curr = parent;
                    temp_path.push_front(self.vertices[curr]);
                    if curr == source_idx {
                        break;
                    }
                }
                None => return Vec::new(),
            }
        }
        temp_path.into_iter().collect()
    }
}

impl<'a> Default for Graph<'a> {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn test_dijkstras() {
    let mut graph = Graph::new();
    graph.add_edge("a", "b", 7);
    graph.add_edge("b", "c", 10);
    graph.add_edge("c", "d", 5);
    graph.add_edge("a", "d", 30);
    graph.add_edge("y", "z", 10); //Disconnected from the rest

    assert_eq!(graph.dijkstra("a", "d"), vec!["a", "b", "c", "d"]);
    assert!(graph.dijkstra("a", "y").is_empty());
    assert!(graph.dijkstra("e", "y").is_empty());
    assert!(graph.dijkstra("a", "e").is_empty());
}

fn main() {
    let mut graph = Graph::new();
    graph.add_edge("a", "b", 7);
    graph.add_edge("a", "c", 9);
    graph.add_edge("a", "f", 14);
    graph.add_edge("b", "c", 10);
    graph.add_edge("b", "d", 15);
    graph.add_edge("c", "d", 11);
    graph.add_edge("c", "f", 2);
    graph.add_edge("d", "e", 6);
    graph.add_edge("e", "f", 9);

    let path = graph.dijkstra("a", "e");
    println!("Path is: {:?}", path);
}

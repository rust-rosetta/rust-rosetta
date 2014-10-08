// Implements http://rosettacode.org/wiki/Dijkstra's_algorithm

use std::collections::{HashMap, PriorityQueue, DList, Deque};
use std::collections::hashmap::{Occupied, Vacant};
use std::uint;

type Node = uint;
type Cost = uint;
type Edge = (Node, Node);


/// The DistPair struct is for the Priority Queue.
#[deriving(Eq, PartialEq)]
struct DistPair(Node, Cost);
impl Ord for DistPair{
    fn cmp(&self, other:&DistPair) -> Ordering{
        let DistPair(_, dist_a) = *self;
        let DistPair(_, dist_b) = *other;
        dist_b.cmp(&dist_a) //Intentionally reversed
    }
}

impl PartialOrd for DistPair{
    fn partial_cmp(&self, other:&DistPair) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Graph structure, represented as an Adjancency List.
struct Graph<'a>{
    vertices: Vec<&'a str>,
    adj_list: Vec<Vec<Node>>,
    costs: HashMap<Edge, Cost>,
}

impl<'a> Graph<'a> {
    fn new() -> Graph<'a>{
        let vertices:Vec<&str> = Vec::new();
        let adj_list:Vec<Vec<Node>> = Vec::new();
        let costs:HashMap<Edge, Cost> = HashMap::new();

        Graph{vertices: vertices, adj_list: adj_list, costs: costs}
    }

    /// Returns the index of the vertex, or None if vertex 
    /// not found.
    fn vertex_index(&self, vertex: &str)->Option<Node>{
        for (idx, &v) in self.vertices.iter().enumerate(){
            if v == vertex{return Some(idx)}
        }
        return None
    }

    /// Returns the index of the vertex. If vertex is not found, inserts
    /// the vertex.
    fn get_or_insert_vertex(&mut self, vertex: &'a str) -> Node {
        
        let new_idx = match self.vertex_index(vertex){
            Some(idx) => idx,
            None => {
                let new_str = vertex;
                self.adj_list.push(Vec::new());
                self.vertices.push(new_str);
                self.vertices.len()-1
            }
        };
        new_idx
    }
    
    /// Adds the given edge to the graph.
    fn add_edge(&mut self, from: &'a str, to: &'a str, cost: uint){
        let from_idx = self.get_or_insert_vertex(from);
        let to_idx = self.get_or_insert_vertex(to);

        match self.costs.entry((from_idx, to_idx)){
            Vacant(entry)   => {
                self.adj_list.get_mut(from_idx).push(to_idx);
                entry.set(cost);
            },
            Occupied(entry) => {
                *entry.into_mut() = cost;
            }
        };
    }

    /// Implements Djikstra's Algorithm. This uses a Priority Queue to 
    /// determine which vertex to visit first. Terminates on discovering 
    /// the target vertex.
    ///
    /// Returns vector of vertices representing the path, or an empty vector
    /// if there's no path, or if the source or target is not in the graph.
    fn dijkstra(&self, source: &str, target: &str)->Vec<&str>{
        let num_vert = self.vertices.len();
        let mut dist:Vec<uint> = Vec::from_elem(num_vert, uint::MAX); //Close enough to infinity
        let mut prev:HashMap<Node, Node> = HashMap::new();
        let mut queue:PriorityQueue<DistPair> = PriorityQueue::new();

        let source_idx = match self.vertex_index(source){
            Some(idx) => idx,
            None      => return Vec::new() // Source not in graph, return empty path.
        };

        let target_idx = match self.vertex_index(target){
            Some(idx) => idx,
            None      => return Vec::new() // Target not in graph, return empty path.
        };

        *dist.get_mut(source_idx) = 0u;
        queue.push(DistPair(source_idx, dist[source_idx]));

        loop{
            match queue.pop(){
                None     => break,
                Some(DistPair(u, dist_u)) => {
                    for &v in self.adj_list[u].iter(){
                        let cost_uv = match self.costs.find(&(u, v)){
                            Some(&x) => x,
                            None    => uint::MAX,
                        };
                        let alt = dist_u + cost_uv;
                        if alt < dist[v] {
                            match prev.entry(v){
                                Vacant(entry) => {entry.set(u);},
                                Occupied(entry) => {
                                    *entry.into_mut() = u;
                                }
                            }
                            *dist.get_mut(v) = alt;
                            queue.push(DistPair(v, dist[v]));
                        }
                        if v == target_idx {break;}
                    }   
                }
            };
        }
        let mut temp_path:DList<&str> = DList::new();
        let mut curr = target_idx;
        temp_path.push_front(self.vertices[curr]);
        loop{
            match prev.find(&curr){
                Some(&parent) => {
                    curr = parent;
                    temp_path.push_front(self.vertices[curr]);
                    if curr == source_idx {break;}
                },
                None => return Vec::new(),
            }
        }
        return temp_path.into_iter().collect();
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
    assert_eq!(graph.dijkstra("a", "y"), vec![]);
    assert_eq!(graph.dijkstra("e", "y"), vec![]);
    assert_eq!(graph.dijkstra("a", "e"), vec![]);
}

#[cfg(not(test))]
fn main(){
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
    println!("Path is: {}", path);
}


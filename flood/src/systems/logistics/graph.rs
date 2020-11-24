use std::collections::HashMap;
use std::collections::hash_map::Iter;

pub type NodeId = u32;
pub type SubnetId = u32;

pub struct Empty;

pub struct Node<T> {
  id: NodeId,
  subnet: SubnetId,
  data: T,
}

/// Representing edges as adjacency list
pub struct Graph<T, E> {
  next_id: NodeId,
  nodes: HashMap<NodeId, Node<T>>,
  edges: HashMap<NodeId, HashMap<NodeId, E>>,
}

impl<V, E> Graph<V, E> {
  pub fn new() -> Self {
    Graph {
      next_id: 0,
      nodes: HashMap::new(),
      edges: HashMap::new(),
    }
  }

  pub fn next_id(&mut self) -> NodeId {
    self.next_id += 1;
    self.next_id
  }

  // Vertex Operations

  pub fn add_node(&mut self, id: NodeId, data: V) -> NodeId {
    let new_node = Node {
      id,
      subnet: self.next_id(),
      data
    };
    self.nodes.insert(id, new_node);
    id
  }

  pub fn node(&self, id: NodeId) -> Option<&Node<V>> {
    self.nodes.get(&id)
  }

  // Edge Operations

  fn assert_nodes_exist(&self, from: NodeId, to: NodeId) {
    assert!(self.nodes.contains_key(&from));
    assert!(self.nodes.contains_key(&to));
  }

  pub fn add_edge(&mut self, from: NodeId, to: NodeId, data: E) {
    self.assert_nodes_exist(from, to);
    let adjacency_list = self.edges.entry(from).or_default();
    adjacency_list.insert(to, data);
  }

  pub fn edge(&self, from: NodeId, to: NodeId) -> Option<&E> {
    self.assert_nodes_exist(from, to);
    self.edges.get(&from)
      .and_then(|adj_list| {
        adj_list.get(&to)
      })
  }

  pub fn connected(&self, from: NodeId, to: NodeId) -> bool {
    self.edge(from, to).is_some()
  }

  pub fn edges(&self, vertex: NodeId) -> Option<Iter<NodeId, E>> {
    self.edges.get(&vertex).map(|adj_list| { adj_list.iter() })
  }
}

#[cfg(test)]
mod tests {
  use crate::systems::logistics::graph::{Graph, Empty};

  #[test]
  fn add_verticies() {
    let mut graph: Graph<u32, u32> = Graph::new();
    let v0 = graph.next_id();
    graph.add_node(v0, 72);
    let v1 = graph.next_id();
    graph.add_node(v1, 73);
    let v2 = graph.next_id();
    graph.add_node(v2, 74);

    assert_eq!(graph.node(v0).unwrap().data, 72);
    assert_eq!(graph.node(v1).unwrap().data, 73);
    assert_eq!(graph.node(v2).unwrap().data, 74);
  }

  #[test]
  fn connected() {
    let mut graph: Graph<Empty, u32> = Graph::new();
    let v0 = graph.next_id();
    graph.add_node(v0, Empty);
    let v1 = graph.next_id();
    graph.add_node(v1, Empty);
    let v2 = graph.next_id();
    graph.add_node(v2, Empty);

    graph.add_edge(v0, v1, 10);
    graph.add_edge(v1, v2, 20);
    graph.add_edge(v2, v1, 30);

    assert_eq!(graph.edge(v0, v1), Some(&10));
    assert_eq!(graph.edge(v1, v0), None);

    assert!(graph.connected(v0, v1));
    assert!(!graph.connected(v1, v0));
    assert!(!graph.connected(v2, v0));
    assert!(graph.connected(v2, v1));
  }

  #[test]
  fn edge_iterator() {
    let mut graph: Graph<Empty, u32> = Graph::new();
    let v0 = graph.next_id();
    graph.add_node(v0, Empty);
    let v1 = graph.next_id();
    graph.add_node(v1, Empty);
    let v2 = graph.next_id();
    graph.add_node(v2, Empty);

    graph.add_edge(v0, v1, 10);
    graph.add_edge(v1, v2, 20);
    graph.add_edge(v2, v1, 30);

    if let Some(edges) = graph.edges(v0) {
      let edge_total = edges.count();
      assert_eq!(edge_total, 1);
    } else {
      assert!(false)
    }
  }
}
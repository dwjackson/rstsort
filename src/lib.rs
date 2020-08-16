mod arena;

use std::collections::HashMap;
use arena::*;

pub type NodeHandle = SlotHandle;

pub struct Node<T> {
    data: T,
    edges: Vec<NodeHandle>,
}

impl<T> Node<T> {
    pub fn data(&self) -> &T {
        &self.data
    }
}

pub struct Digraph<T> {
    nodes: Arena<Node<T>>,
}

impl<T> Digraph<T> {
    pub fn new() -> Digraph<T> {
        Digraph {
            nodes: Arena::new(),
        }
    }

    pub fn node_count(&self) -> usize {
        self.nodes.count()
    }

    pub fn add_node(&mut self, data: T) -> NodeHandle {
        let node = Node {
            data,
            edges: Vec::new(),
        };
        self.nodes.add(node) 
    }

    pub fn node(&self, handle: NodeHandle) -> Option<&Node<T>> {
        self.nodes.get(handle) 
    }

    pub fn remove_node(&mut self, handle: NodeHandle) {
        self.nodes.remove(handle);
    }

    pub fn add_edge(&mut self, h1: NodeHandle, h2: NodeHandle) {
        if let Some(node) = self.nodes.get_mut(h1) {
            node.edges.push(h2);
        }
    }

    pub fn tsort(&self) -> Vec<NodeHandle> {
        let mut sorted = Vec::new();
        for h in self.nodes.iter() {
            self.tsort_internal(h, &mut sorted);
        }
        sorted.reverse();
        sorted
    }

    fn tsort_internal(&self, h: NodeHandle, sorted: &mut Vec<NodeHandle>) {
        if let Some(node) = self.nodes.get(h) {
            for edge in node.edges.iter() {
                self.tsort_internal(*edge, sorted);
            }
            if !sorted.contains(&h) {
                sorted.push(h);
            }
        }
    }
}

pub fn parse(s: &str) -> Digraph<String> {
    let mut graph = Digraph::new();
    let mut seen: HashMap<String, NodeHandle> = HashMap::new();
    for line in s.split("\n") {
        let mut handles: Vec<NodeHandle> = Vec::new();
        for name in line.split(" ") {
            let node_name = String::from(name);
            if !seen.contains_key(&node_name) {
                let seen_name = node_name.clone();
                let new_handle = graph.add_node(node_name);
                seen.insert(seen_name, new_handle);
            }
            let h = seen.get(name).expect("Missing handle");
            handles.push(*h);
        }
        let outgoing = handles[0];
        for h in handles[1..].iter() {
            graph.add_edge(outgoing, *h);
        }
    }
    graph
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_graph() {
        let graph: Digraph<u32> = Digraph::new();
        assert_eq!(graph.node_count(), 0);
    }

    #[test]
    fn test_add_node() {
        let mut graph = Digraph::new();
        graph.add_node(42);
        assert_eq!(graph.node_count(), 1);
    }

    #[test]
    fn test_get_node_via_handle() {
        let mut graph = Digraph::new();
        let handle = graph.add_node(42);
        match graph.node(handle) {
            Some(node) => {
                assert_eq!(*node.data(), 42);
            },
            None => panic!("No node found"),
        }
    }

    #[test]
    fn test_remove_node() {
        let mut graph = Digraph::new();
        let handle = graph.add_node(42);
        graph.remove_node(handle);
        assert_eq!(graph.node_count(), 0);
    }

    #[test]
    fn test_add_2_then_remove_1_then_get_deleted() {
        let mut graph = Digraph::new();
        let handle = graph.add_node(1);
        let handle2 = graph.add_node(2);
        graph.remove_node(handle);
        match graph.node(handle2) {
            Some(node) => {
                assert_eq!(*node.data(), 2);
            },
            None => {
                panic!("Node not found");
            },
        }
        match graph.node(handle) {
            Some(_) => {
                panic!("Node should not exist");
            },
            None => {},
        }
    }

    #[test]
    fn test_add_edge() {
        let mut graph = Digraph::new();
        let h1 = graph.add_node(1);
        let h2 = graph.add_node(2);
        graph.add_edge(h1, h2);
        match graph.node(h1) {
            None => { panic!("Node not found") },
            Some(node) => {
                assert_eq!(node.edges.len(), 1);
                match graph.node(node.edges[0]) {
                    None => { panic!("Edge not correct"); },
                    Some(n) => {
                        assert_eq!(*n.data(), 2);
                    },
                }
            },
        }
    }

    #[test]
    fn test_topological_sort() {
        let mut graph = Digraph::new();
        let h2 = graph.add_node(2);
        let h1 = graph.add_node(1);
        graph.add_edge(h1, h2);
        let h3 = graph.add_node(3);
        let h4 = graph.add_node(4);
        graph.add_edge(h2, h3);
        graph.add_edge(h1, h4);
        let sorted = graph.tsort();
        assert_eq!(sorted.len(), graph.node_count());
        let values: Vec<i32> = sorted.iter().map(|h| {
            match graph.nodes.get(*h) {
                Some(node_ptr) => {
                    node_ptr.data
                },
                None => -1,
            }
        }).collect();
        assert_eq!(vec![1, 4, 2, 3], values);
    }

    mod parser_tests {
        use super::*;

        #[test]
        fn test_parse_single_node() {
            let s = "a";
            let graph = parse(&s);
            assert_eq!(graph.node_count(), 1);
            let sorted = graph.tsort();
            assert_eq!(sorted.len(), graph.node_count());
            let values: Vec<&str> = sorted.iter().map(|h| {
                match graph.nodes.get(*h) {
                    Some(node_ptr) => {
                        &node_ptr.data
                    },
                    None => "",
                }
            }).collect();
            assert_eq!(vec!["a"], values);
        }

        #[test]
        fn test_parse_single_edge() {
            let s = "a b";
            test_parse(&s, vec!["a", "b"]);
        }

        fn test_parse(s: &str, expected: Vec<&str>) {
            let graph = parse(s);
            assert_eq!(graph.node_count(), expected.len());
            let sorted = graph.tsort();
            assert_eq!(sorted.len(), graph.node_count());
            let values: Vec<&str> = sorted.iter().map(|h| {
                match graph.nodes.get(*h) {
                    Some(node_ptr) => {
                        &node_ptr.data
                    },
                    None => "",
                }
            }).collect();
            assert_eq!(expected, values);
        }

        #[test]
        fn test_parse_several_edges_from_one_node() {
            test_parse("a b c", vec!["a", "c", "b"]);
        }

        #[test]
        fn test_parse_multiple_lines() {
            test_parse("a b\nb c", vec!["a", "b", "c"]);
        }
    }
}

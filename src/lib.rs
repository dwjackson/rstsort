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

    pub fn tsort(&self) -> Result<Vec<NodeHandle>, TopologicalSortError> {
        let mut sorted = Vec::new();
        let mut seen = HashMap::new();
        for h in self.nodes.iter() {
            self.tsort_internal(h, &mut sorted, &mut seen)?;
        }
        sorted.reverse();
        Ok(sorted)
    }

    fn tsort_internal(&self, h: NodeHandle, sorted: &mut Vec<NodeHandle>, seen: &mut HashMap<NodeHandle, SortStatus>) -> Result<(), TopologicalSortError> {
        if let Some(node) = self.nodes.get(h) {
            seen.entry(h).or_insert(SortStatus::Unseen);
            match seen.get(&h).unwrap() {
                SortStatus::Unseen => {
                    seen.insert(h, SortStatus::Seen);
                    for edge in node.edges.iter() {
                        self.tsort_internal(*edge, sorted, seen)?;
                    }
                    sorted.push(h);
                },
                SortStatus::Seen => {
                    return Err(TopologicalSortError::Cycle);
                },
                SortStatus::Processed => {
                    // We've already done this node
                },
            }
            seen.insert(h, SortStatus::Processed);
            Ok(())
        } else {
            Err(TopologicalSortError::MissingNode)
        }
    }
}

#[derive(Debug)]
pub enum TopologicalSortError {
    MissingNode,
    Cycle,
}

enum SortStatus {
    Unseen,
    Seen,
    Processed,
}

pub struct DigraphParser {
    graph: Digraph<String>,
    seen: HashMap<String, NodeHandle>,
}

#[derive(Debug)]
pub enum GraphParseError {
}

impl DigraphParser {
    pub fn new() -> DigraphParser {
        DigraphParser {
            graph: Digraph::new(),
            seen: HashMap::new(),
        }
    }

    pub fn parse(mut self, s: &str) -> Result<Digraph<String>, GraphParseError> {
        for line in s.split("\n") {
            self.parse_line(line);
        }
        Ok(self.graph)
    }

    pub fn parse_line(&mut self, line: &str) {
        let line = line.trim();
        if line.len() == 0 {
            // Skip blank lines
            return;
        }
        let mut handles: Vec<NodeHandle> = Vec::new();
        for name in line.split(" ") {
            let node_name = String::from(name);
            if !self.seen.contains_key(&node_name) {
                let seen_name = node_name.clone();
                let new_handle = self.graph.add_node(node_name);
                self.seen.insert(seen_name, new_handle);
            }
            let h = self.seen.get(name).expect("Missing handle");
            handles.push(*h);
        }
        let outgoing = handles[0];
        for h in handles[1..].iter() {
            self.graph.add_edge(outgoing, *h);
        }
    }

    pub fn graph(self) -> Digraph<String> {
        self.graph
    }
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
        match graph.tsort() {
            Ok(sorted) => {
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
            },
            Err(err) => {
                panic!("{:?}", err);
            },
        }
    }

    #[test]
    fn test_topological_sort_with_cycle() {
        let mut graph = Digraph::new();
        let h1 = graph.add_node(1);
        let h2 = graph.add_node(2);
        graph.add_edge(h1, h2);
        graph.add_edge(h2, h1);
        match graph.tsort() {
            Ok(_) => {
                panic!("Sort should fail with a cycle");
            },
            Err(_) => {},
        }
    }

    mod parser_tests {
        use super::*;

        #[test]
        fn test_parse_single_node() {
            let s = "a";
            let parser = DigraphParser::new();
            let graph = parser.parse(s).expect("Parse failed");
            assert_eq!(graph.node_count(), 1);
            match graph.tsort() {
                Ok(sorted) => {
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
                },
                Err(err) => {
                    panic!("{:?}", err);
                },
            }
        }

        #[test]
        fn test_parse_single_edge() {
            let s = "a b";
            test_parse(&s, vec!["a", "b"]);
        }

        fn test_parse(s: &str, expected: Vec<&str>) {
            let parser = DigraphParser::new();
            let graph = parser.parse(s).expect("Parse failed");
            assert_eq!(graph.node_count(), expected.len());
            match graph.tsort() {
                Ok(sorted) => {
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
                },
                Err(err) => {
                    panic!("{:?}", err);
                },
            }
        }

        #[test]
        fn test_parse_single_edge_with_trailing_newline() {
            test_parse("a b\n", vec!["a", "b"]);
        }

        #[test]
        fn test_parse_single_edge_with_trailing_space() {
            test_parse("a b ", vec!["a", "b"]);
        }

        #[test]
        fn test_parse_several_edges_from_one_node() {
            test_parse("a b c", vec!["a", "c", "b"]);
        }

        #[test]
        fn test_parse_multiple_lines() {
            test_parse("a b\nb c", vec!["a", "b", "c"]);
        }

        #[test]
        fn test_parse_multiple_lines_with_blanks() {
            test_parse("a b\n\nb c\n", vec!["a", "b", "c"]);
        }
    }
}

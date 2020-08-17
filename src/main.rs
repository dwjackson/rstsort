use rstsort::{DigraphParser, TopologicalSortError};
use std::io;

fn main() {
    let mut parser = DigraphParser::new();
    let stdin = io::stdin();
    let mut line = String::new();
    loop {
        match stdin.read_line(&mut line) {
            Ok(nbytes) => {
                let end_of_file = nbytes == 0;
                if end_of_file {
                    break;
                }
                parser.parse_line(&line);
                line.clear();
            },
            Err(error) => {
                panic!("{:?}", error);
            }
        }
    }
    let graph = parser.graph();
    match graph.tsort() {
        Ok(sorted) => {
            for handle in sorted.iter() {
                let node = graph.node(*handle).unwrap();
                println!("{}", node.data());
            }
        },
        Err(err) => {
            match err {
                TopologicalSortError::Cycle => {
                    println!("Cannot sort, graph contains a cycle");
                },
                _ => {
                    panic!("{:?}", err);
                }
            }
        },
    }
}

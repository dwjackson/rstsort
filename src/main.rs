use rstsort::{DigraphParser, TopologicalSortError};
use std::io;
use std::env;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    let stdin = io::stdin();
    
    let mut input: Box<dyn io::BufRead> = if args.len() > 1 {
        let file_name = &args[1];
        let file = match File::open(file_name) {
            Ok(file) => file,
            Err(err) => panic!("Could not open file {}: {}", file_name, err),
        };
        Box::new(io::BufReader::new(file))
    } else {
        Box::new(stdin.lock())
    };

    let mut parser = DigraphParser::new();
    let mut line = String::new();
    loop {
        match input.read_line(&mut line) {
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

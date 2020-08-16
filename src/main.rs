use rstsort::DigraphParser;
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
    let sorted = graph.tsort();
    for handle in sorted.iter() {
        let node = graph.node(*handle).unwrap();
        println!("{}", node.data());
    }
}

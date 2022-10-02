use lib::Graph;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = if args.len() > 1 {&args[1]} else {"test.tgf"};

    let s = std::fs::read_to_string(filename).unwrap();
    
    let g = Graph::<u32>::unmarshal(&s);
    for node in g.get_all_nodes() {
        print!("node id: {}, neighbors: [", node.id);
        for nbr in g.get_connected(node) {
            print!(" {}", nbr.id);
        }
        println!(" ], value: {}", node.value);
    }
}
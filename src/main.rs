mod bfs;
mod dfs;
mod graph;

use bfs::bfs;
use dfs::dfs;
use graph::Graph;

fn setup() -> Option<Graph<usize>> {
    let mut g = Graph::new(0..10);
    
    g.connect(0, 1);
    g.connect(1, 5);
    g.connect(0, 2);

    Some(g)
}

fn main() {
    let mut g = match setup() {
        Some(g) => g,
        None => panic!("Failed to set up!")
    };

    match dfs(&mut g, 0, &mut Vec::new()) {
        Ok(_) => println!("DFS OK"),
        Err(error) => println!("DFS not OK: {}", error)
    }

    /*g.reset();

    match bfs(&mut g, 0) {
        Ok(_) => println!("BFS OK"),
        Err(error) => println!("BFS not OK: {}", error)
    }*/

    println!("{:?}", g)
}

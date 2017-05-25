mod bfs;
mod dfs;
mod graph;

use bfs::bfs;
use dfs::dfs;
use graph::Graph;

fn setup() -> Result<Graph<usize>, &'static str> {
    let mut g = Graph::new(0..9);
    
    g.connect((0, 1))?;
    g.connect((1, 2))?;
    g.connect((2, 3))?;
    g.connect((3, 4))?;
    g.connect((1, 5))?;
    g.connect((4, 5))?;
    g.connect((0, 6))?;
    g.connect((0, 7))?;
    g.connect((6, 7))?;
    g.connect((0, 8))?;
    g.connect((7, 8))?;

    Ok(g)
}

fn main() {
    let mut g = match setup() {
        Ok(g) => g,
        Err(e) => panic!(e)
    };

    let mut g2 = g.clone();

    match dfs(&mut g, 0, &mut Vec::new()) {
        Ok(_) => println!("DFS OK"),
        Err(error) => println!("DFS not OK: {}", error)
    }

    match bfs(&mut g2, 0) {
        Ok(_) => println!("BFS OK"),
        Err(error) => println!("BFS not OK: {}", error)
    }

    println!("{:?}", g)
}

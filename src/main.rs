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
        Ok(result) => println!("DFS results: {:?}", result),
        Err(error) => println!("DFS failed: {}", error)
    }

    match bfs(&mut g2, 0) {
        Ok(result) => println!("BFS results: {:?}", result),
        Err(error) => println!("BFS not OK: {}", error)
    }
}

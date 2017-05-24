use graph::Graph;

pub fn dfs<T>(graph: &mut Graph<T>, node: usize, path: &mut Vec<usize>) -> Result<(), &'static str> {
    graph.mark(node)?;
    graph.setPath(node, &path);
    path.push(node);

    let neighbours = graph.neighbours(node)?;
    for neighbour in neighbours {
        if graph.is_marked(neighbour)? {
            println!("A! {:?}", path)
        } else {
            println!("B!");
            dfs(graph, neighbour, &mut path.clone())?
        }
    }

    Ok(())
}
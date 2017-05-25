use graph::Graph;

fn get_cycle<T>(graph: &Graph<T>, nodes: (usize, usize)) -> Result<Vec<usize>, &'static str> {
    let mut ret = Vec::new();

    let stacks = (graph.get_path(nodes.0)?, graph.get_path(nodes.1)?);

    let mut iter = stacks.1.iter().peekable();
    for i in stacks.0 {
        let mut cont = false;
        match iter.peek() {
            Some(j) => {
                if **j == i {
                    cont = true;
                } else {
                    ret.push(i)
                }
            }
            None => ret.push(i)
        }
        if cont {
            iter.next();
        }
    }

    for i in iter {
        ret.push(*i)
    }

    ret.push(nodes.0);

    Ok(ret)
}

pub fn dfs<T>(graph: &mut Graph<T>, node: usize, path: &mut Vec<usize>) -> Result<Vec<Vec<usize>>, &'static str> {
    let mut ret = Vec::new();

    graph.mark(node)?;
    graph.set_path(node, &path)?;

    let mut candidate = 0;
    // For every neighbour
    while let Some(neighbour) = graph.next_neighbour(node, candidate) {
        // Consume the edge
        graph.disconnect((node, neighbour))?;
        // A cycle?
        if graph.is_marked(neighbour)? {
            let cycle = get_cycle(graph, (node, neighbour))?;
            ret.push(cycle);
        } else {
            let mut path = path.clone();
            path.push(node);
            ret.extend(dfs(graph, neighbour, &mut path)?);
        }
        candidate = neighbour + 1;
    }

    Ok(ret)
}
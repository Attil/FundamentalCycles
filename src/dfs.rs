use std::collections::VecDeque;

use graph::Graph;

pub fn dfs<T>(graph: &mut Graph<T>, node: usize) -> Result<Vec<Vec<usize>>, &'static str> {
    let mut ret = Vec::new();

    let mut queue = VecDeque::new();

    graph.set_path(node, &vec![])?;
    graph.mark(node)?;

    queue.push_back(node);

    while !queue.is_empty() {
        let current = match queue.pop_front() {
            Some(value) => value,
            None => return Err("Logic doesn't work, someone the universe fixing team")
        };

        // For every neighbour
        let neighbours = &graph.nodes[current].neighbours.clone();
        for (neighbour, _) in neighbours {
            graph.disconnect((current, *neighbour))?;
            if graph.is_marked(*neighbour)? {
                let cycle = graph.get_cycle((current, *neighbour))?;
                ret.push(cycle);
            } else {
                graph.mark(*neighbour)?;

                let mut path = graph.get_path(current).clone()?;
                path.push(current);
                graph.set_path(*neighbour, &path)?;

                queue.push_front(*neighbour);
            }
        }
    }

    Ok(ret)
}
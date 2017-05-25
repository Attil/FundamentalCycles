use std::collections::VecDeque;

use graph::Graph;

fn get_cycle<T>(graph: &Graph<T>, nodes: (usize, usize)) -> Result<Vec<usize>, &'static str> {
    let mut ret = Vec::new();

    let stacks = (graph.get_path(nodes.0)?, graph.get_path(nodes.1)?);

    let mut iter = stacks.0.iter().peekable();
    let mut last_mutual = None;
    for i in stacks.1 {
        let mut cont = false;
        match iter.peek() {
            Some(j) => {
                if **j == i {
                    cont = true;
                    last_mutual = Some(i);
                } else {
                    ret.push(i);
                }
            },
            None => ret.push(i)
        }
        if cont {
            iter.next();
        }
    }

    if let Some(i) = last_mutual {
        ret.push(i);
    }
    for i in iter {
        ret.push(*i);
    }
    ret.push(nodes.0);
    ret.push(nodes.1);

    Ok(ret)
}

pub fn bfs<T>(graph: &mut Graph<T>, node: usize) -> Result<Vec<Vec<usize>>, &'static str> {
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

        let mut candidate = 0;
        // For every neighbour
        while let Some(neighbour) = graph.next_neighbour(current, candidate) {
            graph.disconnect((current, neighbour))?;
            if graph.is_marked(neighbour)? {
                let cycle = get_cycle(graph, (current, neighbour))?;
                ret.push(cycle);
            } else {
                graph.mark(neighbour)?;

                let mut path = graph.get_path(current).clone()?;
                path.push(current);
                graph.set_path(neighbour, &path)?;

                queue.push_back(neighbour);
            }

            candidate = neighbour + 1;
        }
    }

    Ok(ret)
}
use std::collections::VecDeque;

use graph::Graph;

pub fn bfs<T>(graph: &mut Graph<T>, node: usize) -> Result<(), &'static str> {
    let mut queue = VecDeque::new();

    queue.push_back(node);

    while !queue.is_empty() {
        let current = match queue.pop_front() {
            Some(value) => value,
            None => return Err("Logic doesn't work, someone the universe fixing team")
        };

        graph.mark(current)?;

        let neighbours = graph.neighbours(current)?;
        for neighbour in neighbours {
            if graph.is_marked(neighbour)? {
                println!("A!")
            } else {
                println!("Operating on {}, adding {}!", current, neighbour);
                queue.push_back(neighbour)
            }
        }
    }

    Ok(())
}
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub data: T,
    pub visited: bool,
    pub path: Vec<usize>,
    pub neighbours: HashMap<usize, ()>
}

#[derive(Clone, Debug)]
pub struct Graph<T> {
    pub nodes: Vec<Node<T>>
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node {
            data: value,
            visited: false,
            path: Vec::new(),
            neighbours: HashMap::new()
        }
    }
}

impl<T> Graph<T> {
    pub fn new<U>(iterator: U) -> Graph<T> where U: Iterator<Item = T> {
        let mut nodes = Vec::new();
        for i in iterator {
            nodes.push(Node::new(i));
        }

        Graph {
            nodes: nodes
        }
    }

    pub fn connect(&mut self, pos: (usize, usize)) -> Result<(), &'static str> {
        if pos.0 >= self.nodes.len() || pos.1 >= self.nodes.len() {
            return Err("Out of bounds");
        }

        self.nodes[pos.0].neighbours.insert(pos.1, ());
        self.nodes[pos.1].neighbours.insert(pos.0, ());

        Ok(())
    }

    pub fn disconnect(&mut self, pos: (usize, usize)) -> Result<(), &'static str> {
        if pos.0 >= self.nodes.len() || pos.1 >= self.nodes.len() {
            return Err("Out of bounds");
        }

        self.nodes[pos.0].neighbours.remove(&pos.1);
        self.nodes[pos.1].neighbours.remove(&pos.0);

        Ok(())
    }

    pub fn mark(&mut self, i: usize) -> Result<(), &'static str> {
        if i >= self.nodes.len() {
            return Err("Out of bounds");
        }
        self.nodes[i].visited = true;

        Ok(())
    }

    pub fn is_marked(&self, i: usize) -> Result<bool, &'static str> {
        if i >= self.nodes.len() {
            return Err("Out of bounds");
        }

        Ok(self.nodes[i].visited)
    }

    /*pub fn next_neighbour(&self, node: usize, first_candidate: usize) -> Option<usize> {
        for i in first_candidate..self.nodes.len() {
            if i == node {
                continue;
            }
            let connected = match self.connected((node, i)) {
                Ok(x) => x,
                Err(_) => return None
            };
            if connected {
                return Some(i)
            }
        }
        None
    }*/

    pub fn set_path(&mut self, i: usize, path: &Vec<usize>) -> Result<(), &'static str> {
        if i >= self.nodes.len() {
            return Err("Out of bounds");
        }

        self.nodes[i].path = path.clone();

        Ok(())
    }

    pub fn get_path(&self, i: usize) -> Result<Vec<usize>, &'static str> {
        if i >= self.nodes.len() {
            return Err("Out of bounds");
        }

        Ok(self.nodes[i].path.clone())
    }

    pub fn get_cycle(&self, nodes: (usize, usize)) -> Result<Vec<usize>, &'static str> {
        let mut ret = Vec::new();

        let stacks = (self.get_path(nodes.0)?, self.get_path(nodes.1)?);

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
}
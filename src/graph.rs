#[derive(Clone, Debug)]
pub struct Node<T> {
    pub data: T,
    pub visited: bool,
    pub path: Vec<usize>
}

#[derive(Clone, Debug)]
pub struct Graph<T> {
    pub nodes: Vec<Node<T>>,
    pub edges: Vec<bool>
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node {
            data: value,
            visited: false,
            path: Vec::new()
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
            edges: vec![false; (nodes.len()*(nodes.len()-1))/2],
            nodes: nodes
        }
    }

    fn get_idx(&self, pos: (usize, usize)) -> usize {
        let ordered;
        if pos.0 < pos.1 {
            ordered = pos
        } else {
            ordered = (pos.1, pos.0)
        }
        let nodes = self.nodes.len();
        let size = ((nodes)*(nodes-1))/2;

        let reduction = ((nodes-ordered.0)*(nodes-ordered.0-1))/2;
        size - reduction + (nodes - ordered.1 - 1)
    }

    pub fn connect(&mut self, pos: (usize, usize)) -> Result<(), &'static str> {
        if pos.0 >= self.nodes.len() || pos.1 >= self.nodes.len() {
            return Err("Out of bounds");
        }

        let idx = self.get_idx(pos);
        self.edges[idx] = true;

        Ok(())
    }

    pub fn disconnect(&mut self, pos: (usize, usize)) -> Result<(), &'static str> {
        if pos.0 >= self.nodes.len() || pos.1 >= self.nodes.len() {
            return Err("Out of bounds");
        }

        let idx = self.get_idx(pos);
        self.edges[idx] = false;

        Ok(())
    }

    pub fn connected(&self, pos: (usize, usize)) -> Result<bool, &'static str> {
        if pos.0 >= self.nodes.len() || pos.1 >= self.nodes.len() {
            return Err("Out of bounds");
        }

        let idx = self.get_idx(pos);

        Ok(self.edges[idx])
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

    pub fn next_neighbour(&self, node: usize, first_candidate: usize) -> Option<usize> {
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
    }

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
}
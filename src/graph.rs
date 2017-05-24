#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    pub visited: bool,
    pub neighbours: Vec<usize>,
    pub path: Vec<usize>
}

#[derive(Debug)]
pub struct Graph<T> {
    pub nodes: Vec<Node<T>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node {
            data: value,
            visited: false,
            neighbours: Vec::new(),
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
            nodes: nodes,
        }
    }

    pub fn connect(&mut self, i: usize, j: usize) -> Result<(), &'static str> {
        if i >= self.nodes.len() || j >= self.nodes.len() {
            return Err("Out of bounds");
        }

        self.nodes[i].neighbours.push(j);
        self.nodes[j].neighbours.push(i);

        Ok(())
    }

    pub fn mark(&mut self, i: usize) -> Result<(), &'static str> {
        if i >= self.nodes.len() {
            return Err("Out of bounds");
        }

        println!("{}", i);
        self.nodes[i].visited = true;

        Ok(())
    }

    pub fn is_marked(&self, i: usize) -> Result<bool, &'static str> {
        if i >= self.nodes.len() {
            return Err("Out of bounds");
        }

        Ok(self.nodes[i].visited)
    }

    pub fn neighbours(&self, i: usize) -> Result<Vec<usize>, &'static str> {
        if i >= self.nodes.len() {
            return Err("Out of bounds");
        }

        Ok(self.nodes[i].neighbours.clone())
    }

    pub fn reset(&mut self) {
        for node in &mut self.nodes {
            node.visited = false;
            node.path = Vec::new()
        }
    }

    pub fn setPath(&mut self, i: usize, path: &Vec<usize>) -> Result<(), &'static str> {
        if i >= self.nodes.len() {
            return Err("Out of bounds");
        }

        self.nodes[i].path = path.clone();

        Ok(())
    }
}
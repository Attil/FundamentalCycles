#![feature(test)]
extern crate test;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

mod bfs;
mod dfs;
mod graph;

mod file;

use std::fs::File;
use std::io::prelude::*;

use bfs::bfs;
use dfs::dfs;
use graph::Graph;

use file::TestFile;

fn setup(test: TestFile) -> Result<Graph<usize>, &'static str> {
    let mut g = Graph::new(0..test.num);
    
    for edge in test.edges {
        g.connect((edge.0, edge.1))?;
    }

    Ok(g)
}

fn load_file(filename: &'static str) -> Result<Graph<usize>, &'static str> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => return Err("Error while opening file")
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => {},
        Err(_) => return Err("Error while reading file")
    };

    let test = match serde_yaml::from_str(&content) {
        Ok(test) => test,
        Err(_) => return Err("Error while parsing tests")
    };

    setup(test)
}

fn main() {
    let mut g = match load_file("tests/simple.test") {
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

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn empty_dfs() {
        let mut g = load_file("tests/empty.test").unwrap();
        let res = dfs(&mut g, 0, &mut Vec::new()).unwrap();
        assert_eq!(res, Vec::<Vec<usize>>::new());
    }

    #[test]
    fn empty_bfs() {
        let mut g = load_file("tests/empty.test").unwrap();
        let res = bfs(&mut g, 0).unwrap();
        assert_eq!(res, Vec::<Vec<usize>>::new());
    }

    #[test]
    fn simple_dfs() {
        let mut g = load_file("tests/simple.test").unwrap();
        let res = dfs(&mut g, 0, &mut Vec::new()).unwrap();
        assert_eq!(res, vec![vec![1, 2, 3, 4, 5], vec![0, 1, 2, 3, 4, 5, 8], vec![0, 1, 2, 3, 4, 5, 8, 7], vec![0, 1, 2, 3, 4, 5, 8, 7, 6]]);
    }

    #[test]
    fn simple_bfs() {
        let mut g = load_file("tests/simple.test").unwrap();
        let res = bfs(&mut g, 0).unwrap();
        assert_eq!(res, vec![vec![0, 6, 7], vec![0, 7, 8], vec![1, 0, 8, 5], vec![5, 1, 2, 3, 4]]);
    }

    #[test]
    fn double_dfs() {
        let mut g = load_file("tests/double.test").unwrap();
        let res = dfs(&mut g, 0, &mut Vec::new()).unwrap();
        assert_eq!(res, vec![vec![1, 2, 3, 4, 5], vec![0, 6, 7], vec![0, 6, 7, 8]]);
    }

    #[test]
    fn double_bfs() {
        let mut g = load_file("tests/double.test").unwrap();
        let res = bfs(&mut g, 0).unwrap();
        assert_eq!(res, vec![vec![0, 6, 7], vec![0, 7, 8], vec![5, 1, 2, 3, 4]]);
    }

    #[bench]
    fn bench_dfs(b: &mut Bencher) {
        let g = load_file("tests/performance.test").unwrap();
        b.iter(|| {
            dfs(&mut g.clone(), 0, &mut Vec::new()).unwrap();
        })
    }

    #[bench]
    fn bench_bfs(b: &mut Bencher) {
        let g = load_file("tests/performance.test").unwrap();
        b.iter(|| {
            bfs(&mut g.clone(), 0).unwrap();
        })
    }

    #[bench]
    fn bench_dfs_list(b: &mut Bencher) {
        let g = load_file("tests/performance_list.test").unwrap();
        b.iter(|| {
            dfs(&mut g.clone(), 0, &mut Vec::new()).unwrap();
        })
    }

    #[bench]
    fn bench_bfs_list(b: &mut Bencher) {
        let g = load_file("tests/performance_list.test").unwrap();
        b.iter(|| {
            bfs(&mut g.clone(), 0).unwrap();
        })
    }

    #[bench]
    fn bench_dfs_big_list(b: &mut Bencher) {
        let g = load_file("tests/performance_big_list.test").unwrap();
        b.iter(|| {
            dfs(&mut g.clone(), 0, &mut Vec::new()).unwrap();
        })
    }

    #[bench]
    fn bench_bfs_big_list(b: &mut Bencher) {
        let g = load_file("tests/performance_big_list.test").unwrap();
        b.iter(|| {
            bfs(&mut g.clone(), 0).unwrap();
        })
    }
}
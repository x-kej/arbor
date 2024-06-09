use ahash::{HashMap, HashMapExt};
use core::hash::Hash;
use std::collections::VecDeque;

pub trait State: Hash + Clone + PartialEq + Eq {
    fn neighbors(&self) -> Vec<Self>;
    fn is_goal(&self) -> bool;
}

pub struct Tree<T: State> {
    queue: VecDeque<(T, T)>,
    visited: HashMap<T, Option<T>>,
}

impl<T: State> Tree<T> {
    pub fn new(start: &T) -> Tree<T> {
        let mut tree = Tree {
            queue: VecDeque::new(),
            visited: HashMap::new(),
        };
        tree.visited.insert(start.clone(), None);
        for t in start.neighbors().iter() {
            tree.queue.push_back((t.clone(), start.clone()));
        }
        tree
    }

    fn get_path_to_node(&self, node: &T) -> Vec<T> {
        let mut result = Vec::new();
        let mut current = node;
        result.push(current.clone());
        while let Some(c) = self.visited.get(current) {
            if let Some(d) = c {
                current = d;
                result.push(current.clone());
            } else {
                break;
            }
        }
        result.reverse();
        result
    }

    pub fn run(&mut self) -> Option<Vec<T>> {
        while !self.queue.is_empty() {
            let (current, prev) = self.queue.pop_front().unwrap();
            if self.visited.contains_key(&current) {
                continue;
            }
            self.visited.insert(current.clone(), Some(prev));
            if current.is_goal() {
                return Some(self.get_path_to_node(&current));
            }
            for t in current.neighbors() {
                self.queue.push_back((t.clone(), current.clone()));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Hash, Clone, PartialEq, Eq, Debug)]
    struct Towers {
        pegs: Vec<Vec<usize>>,
    }

    impl Towers {
        fn new(pegs: usize, discs: usize) -> Towers {
            let mut result = Towers { pegs: Vec::new() };
            result.pegs.push((0..discs).collect::<Vec<usize>>());
            for _ in 1..pegs {
                result.pegs.push(Vec::new());
            }
            result
        }

        fn move_disc(&self, from: usize, to: usize) -> Option<Towers> {
            if from == to
                || from >= self.pegs.len()
                || to >= self.pegs.len()
                || self.pegs[from].is_empty()
                || (!self.pegs[to].is_empty()
                    && self.pegs[from].last().unwrap() < self.pegs[to].last().unwrap())
            {
                return None;
            }
            let mut result = self.clone();
            let moved = result.pegs[from].pop().unwrap();
            result.pegs[to].push(moved);
            Some(result)
        }
    }

    impl State for Towers {
        fn neighbors(&self) -> Vec<Towers> {
            let mut result = Vec::new();
            for i in 0..self.pegs.len() {
                for j in 0..self.pegs.len() {
                    if let Some(neighbor) = self.move_disc(i, j) {
                        result.push(neighbor);
                    }
                }
            }
            result
        }

        fn is_goal(&self) -> bool {
            for i in 0..(self.pegs.len() - 1) {
                if !self.pegs[i].is_empty() {
                    return false;
                }
            }
            true
        }
    }

    fn hanoi_len(pegs: usize, discs: usize) -> usize {
        let start = Towers::new(pegs, discs);
        let mut tree = Tree::new(&start);
        if let Some(solution) = tree.run() {
            return solution.len() - 1;
        }
        0
    }

    #[test]
    fn test_hanoi() {
        for d in 1..7 {
            let moves = hanoi_len(3, d);
            assert_eq!(2usize.pow(d as u32) - 1, moves);
        }
    }
}

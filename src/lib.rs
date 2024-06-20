use ahash::{HashMap, HashMapExt};
use core::hash::Hash;
use std::collections::VecDeque;
use std::rc::Rc;

pub trait State: Hash + PartialEq + Eq {
    fn neighbors(&self) -> Vec<Rc<Self>>;
    fn is_goal(&self) -> bool;
}

pub struct Tree<T: State> {
    queue: VecDeque<(Rc<T>, Rc<T>)>,
    visited: HashMap<Rc<T>, Option<Rc<T>>>,
}

impl<T: State> Tree<T> {
    pub fn new(start: Rc<T>) -> Tree<T> {
        let mut tree = Tree {
            queue: VecDeque::new(),
            visited: HashMap::new(),
        };
        tree.visited.insert(Rc::clone(&start), None);
        for t in start.neighbors() {
            tree.queue.push_back((t, Rc::clone(&start)));
        }
        tree
    }

    fn get_path_to_node(&self, node: &Rc<T>) -> Vec<Rc<T>> {
        let mut result = Vec::new();
        let mut current = node;
        result.push(Rc::clone(current));
        while let Some(c) = self.visited.get(current) {
            if let Some(d) = c {
                current = d;
                result.push(Rc::clone(current));
            } else {
                break;
            }
        }
        result.reverse();
        result
    }

    pub fn run(&mut self) -> Option<Vec<Rc<T>>> {
        while let Some((current, prev)) = self.queue.pop_front() {
            if self.visited.contains_key(&current) {
                continue;
            }
            self.visited
                .insert(Rc::clone(&current), Some(Rc::clone(&prev)));
            if current.is_goal() {
                return Some(self.get_path_to_node(&current));
            }
            for t in current.neighbors() {
                self.queue.push_back((t, Rc::clone(&current)));
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
        fn neighbors(&self) -> Vec<Rc<Towers>> {
            let mut result = Vec::new();
            for i in 0..self.pegs.len() {
                for j in 0..self.pegs.len() {
                    if let Some(neighbor) = self.move_disc(i, j) {
                        result.push(Rc::new(neighbor));
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
        let mut tree = Tree::new(Rc::new(start));
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

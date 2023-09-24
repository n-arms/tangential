use std::collections::HashSet;
use std::hash::Hash;

pub struct Forest<T> {
    trees: HashSet<T>,
}

impl<T> Default for Forest<T> {
    fn default() -> Self {
        Self {
            trees: HashSet::new(),
        }
    }
}

impl<T: Eq + Hash> Forest<T> {
    pub fn add(&mut self, tree: T) {
        self.trees.insert(tree);
    }
}

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

impl<T: Eq + Hash + Clone> Forest<T> {
    pub fn new(trees: impl Into<HashSet<T>>) -> Self {
        Self {
            trees: trees.into(),
        }
    }

    pub fn add(&mut self, tree: T) {
        self.trees.insert(tree);
    }

    pub fn extend(&mut self, other: Self) {
        self.trees.extend(other.trees);
    }

    pub fn product(woodland: impl AsRef<[Forest<T>]>) -> Forest<Vec<T>> {
        match woodland.as_ref() {
            [] => Forest::new([Vec::new()]),
            [forest] => Forest::new(
                forest
                    .trees
                    .iter()
                    .map(|tree| vec![tree.clone()])
                    .collect::<HashSet<_>>(),
            ),
            [head @ .., tail] => {
                let first = Forest::product(head);

                let trees = first.trees.into_iter().flat_map(|tree| {
                    tail.trees
                        .iter()
                        .map(|tail_tree| {
                            let mut result = tree.clone();
                            result.push(tail_tree.clone());
                            result
                        })
                        .collect::<Vec<_>>()
                });

                Forest::new(trees.collect::<HashSet<_>>())
            }
        }
    }

    pub fn get_parse(&self) -> Option<T> {
        self.trees.iter().next().cloned()
    }

    pub(crate) fn tuple_product<U: Eq + Hash + Clone + 'static>(
        self,
        other: Forest<U>,
    ) -> Forest<(T, U)> {
        let mut result = Forest::default();

        for first in self.trees.iter() {
            for second in other.trees.iter() {
                result.add((first.clone(), second.clone()));
            }
        }

        result
    }
}

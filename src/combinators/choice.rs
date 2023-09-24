use crate::{forest::Forest, parser::Parser};
use std::hash::Hash;

pub struct Choice<T> {
    options: Vec<Box<dyn Parser<Output = T>>>,
}

impl<T: Eq + Hash + 'static> Parser for Choice<T> {
    type Output = T;

    fn parse_null(&self, forest: &mut Forest<Self::Output>) {
        for option in &self.options {
            option.parse_null(forest);
        }
    }

    fn derivative(&self, character: char) -> Box<dyn Parser<Output = Self::Output>> {
        let options = self
            .options
            .iter()
            .map(|option| option.derivative(character))
            .collect();
        Box::new(Choice { options })
    }
}

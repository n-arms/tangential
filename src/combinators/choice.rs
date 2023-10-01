use crate::{forest::Forest, parser::Parser};
use std::hash::Hash;

pub struct Choice<T> {
    options: Vec<Box<dyn Parser<Output = T>>>,
}

impl<T: Eq + Hash + Clone + 'static> Choice<T> {
    pub(crate) fn new(options: Vec<Box<dyn Parser<Output = T>>>) -> Self {
        Self { options }
    }
}

impl<T: Eq + Hash + Clone + 'static> Parser for Choice<T> {
    type Output = T;

    fn parse_null(&self, forest: &mut Forest<Self::Output>) {
        for option in &self.options {
            option.parse_null(forest);
        }
    }

    fn derivative(&self, character: char) -> Box<dyn Parser<Output = Self::Output>> {
        let options: Vec<_> = self
            .options
            .iter()
            .map(|option| option.derivative(character))
            .collect();
        Box::new(Choice { options })
    }

    fn clone_box(&self) -> Box<dyn Parser<Output = Self::Output>> {
        let options: Vec<Box<dyn Parser<Output = T>>> = self
            .options
            .iter()
            .map(|parser| parser.clone_box())
            .collect();
        Box::new(Self { options })
    }
}

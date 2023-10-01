/*
use crate::{forest::Forest, parser::Parser};
use std::hash::Hash;

use super::{choice::Choice, is_null::IsNull};

pub struct Sequence<T> {
    sequence: Vec<Box<dyn Parser<Output = T>>>,
}

impl<T: Eq + Hash + Clone + 'static> Parser for Sequence<T> {
    type Output = Vec<T>;

    fn parse_null(&self, forest: &mut Forest<Self::Output>) {
        parse_null_sequence(self.sequence.iter(), forest)
    }

    fn derivative(&self, character: char) -> Box<dyn Parser<Output = Self::Output>> {
        let mut choices = Vec::new();

        for i in 0..self.sequence.len() {
            let is_null = &self.sequence[..i];
            let has_character = &self.sequence[i];
            let rest = &self.sequence[i + 1..];

            let mut sequence = Vec::new();

            sequence.extend(is_null.iter().map(|parser| IsNull(parser.clone_box())));
        }

        let unboxed_choice: Choice<Vec<T>> = Choice::new(choices);

        todo!()

        //Box::new(Choice::new(choices))
    }

    fn clone_box(&self) -> Box<dyn Parser<Output = Self::Output>> {
        let sequence = self
            .sequence
            .iter()
            .map(|parser| parser.as_ref().clone_box())
            .collect();

        let unboxed: Sequence<T> = Self { sequence };
        todo!()
    }
}

pub(crate) fn parse_null_sequence<T, I>(
    sequence: impl IntoIterator<Item = I>,
    forest: &mut Forest<Vec<T>>,
) where
    T: Eq + Hash + Clone + 'static,
    I: AsRef<dyn Parser<Output = T>>,
{
    let parses: Vec<_> = sequence
        .into_iter()
        .map(|parser| {
            let mut forest = Forest::default();
            parser.as_ref().parse_null(&mut forest);
            forest
        })
        .collect();
    forest.extend(Forest::product(parses))
}

impl<T, I: IntoIterator<Item = Box<dyn Parser<Output = T>>>> From<I> for Sequence<T> {
    fn from(value: I) -> Self {
        Self {
            sequence: value.into_iter().collect(),
        }
    }
}

impl<T: Eq + Hash + Clone + 'static> Sequence<T> {
    pub fn new(sequence: Vec<Box<dyn Parser<Output = T>>>) -> Self {
        Self { sequence }
    }
}
*/

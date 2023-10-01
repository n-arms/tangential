/*

use crate::{forest::Forest, parser::Parser, primitives::void::Void};
use std::hash::Hash;

use super::sequence::parse_null_sequence;
pub struct AllNull<T> {
    parsers: Vec<Box<dyn Parser<Output = T>>>,
}

impl<T: Eq + Hash + Clone + 'static> Parser for AllNull<T> {
    type Output = Vec<T>;

    fn parse_null(&self, forest: &mut Forest<Self::Output>) {
        parse_null_sequence(self.parsers.iter(), forest)
    }

    fn derivative(&self, _: char) -> Box<dyn Parser<Output = Self::Output>> {
        Box::<Void<Vec<T>>>::default()
    }
}
*/

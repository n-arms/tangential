use crate::forest::Forest;
use std::hash::Hash;

pub trait Parser {
    type Output: Eq + Hash;

    fn parse_null(&self, forest: &mut Forest<Self::Output>);
    fn derivative(&self, character: char) -> Box<dyn Parser<Output = Self::Output>>;
    fn compact(&mut self) {}
}

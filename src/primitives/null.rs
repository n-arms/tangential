use std::hash::Hash;

use crate::forest::Forest;
use crate::parser::Parser;

use super::void::Void;

pub struct Null<T>(pub T);

impl<T: Eq + Clone + Hash + 'static> Parser for Null<T> {
    type Output = T;

    fn parse_null(&self, forest: &mut Forest<Self::Output>) {
        forest.add(self.0.clone());
    }

    fn derivative(&self, _: char) -> Box<dyn Parser<Output = Self::Output>> {
        Box::<Void<T>>::default()
    }
}

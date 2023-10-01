use crate::{forest::Forest, parser::Parser, primitives::void::Void};
use std::hash::Hash;

pub struct IsNull<T>(pub Box<dyn Parser<Output = T>>);

impl<T: Eq + Hash + Clone + 'static> Parser for IsNull<T> {
    type Output = T;

    fn parse_null(&self, forest: &mut Forest<Self::Output>) {
        self.0.parse_null(forest);
    }

    fn derivative(&self, _: char) -> Box<dyn Parser<Output = Self::Output>> {
        Box::<Void<T>>::default()
    }

    fn clone_box(&self) -> Box<dyn Parser<Output = Self::Output>> {
        Box::new(Self(self.0.clone_box()))
    }
}

impl<T: Eq + Hash + Clone + 'static> IsNull<T> {
    pub fn new(inner: &dyn Parser<Output = T>) -> Self {
        Self(inner.clone_box())
    }
}

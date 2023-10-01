use crate::{forest::Forest, parser::Parser};
use std::hash::Hash;
use std::marker::PhantomData;

pub struct Void<T>(PhantomData<T>);

impl<T: Eq + Hash + Clone + 'static> Parser for Void<T> {
    type Output = T;

    fn parse_null(&self, _: &mut Forest<Self::Output>) {}

    fn derivative(&self, _: char) -> Box<dyn Parser<Output = T>> {
        Box::new(self.clone())
    }

    fn clone_box(&self) -> Box<dyn Parser<Output = Self::Output>> {
        Box::new(self.clone())
    }
}

impl<T> Clone for Void<T> {
    fn clone(&self) -> Self {
        Void::default()
    }
}

impl<T> Default for Void<T> {
    fn default() -> Self {
        Self(PhantomData::default())
    }
}

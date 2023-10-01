use crate::forest::Forest;
use std::hash::Hash;

pub trait Parser {
    type Output: Eq + Hash + Clone + 'static;

    fn parse_null(&self, forest: &mut Forest<Self::Output>);
    fn derivative(&self, character: char) -> Box<dyn Parser<Output = Self::Output>>;
    fn clone_box(&self) -> Box<dyn Parser<Output = Self::Output>>;
    fn compact(&mut self) {}
}

pub fn parse_iter<O: Eq + Hash + Clone + 'static>(
    parser: &impl Parser<Output = O>,
    iter: impl IntoIterator<Item = char>,
) -> Option<O> {
    let mut parser = parser.clone_box();

    for char in iter.into_iter() {
        parser = parser.derivative(char);
    }

    let mut forest = Forest::default();

    parser.parse_null(&mut forest);

    forest.get_parse()
}

pub fn parse_str<O: Eq + Hash + Clone + 'static>(
    parser: &impl Parser<Output = O>,
    string: impl AsRef<str>,
) -> Option<O> {
    parse_iter(parser, string.as_ref().chars())
}

pub trait CanParse {
    type Output;

    fn parser(self) -> Box<dyn Parser<Output = <Self as CanParse>::Output>>;
}

impl<O, P: Parser<Output = O>> CanParse for P {
    type Output = O;

    fn parser(self) -> Box<dyn Parser<Output = <Self as CanParse>::Output>> {
        self.clone_box()
    }
}

impl<O> CanParse for Box<dyn Parser<Output = O>> {
    type Output = O;

    fn parser(self) -> Box<dyn Parser<Output = <Self as CanParse>::Output>> {
        self
    }
}

use super::null::Null;
use super::void::Void;
use crate::forest::Forest;
use crate::parser::Parser;

#[derive(Copy, Clone)]
pub struct Char(pub char);

impl Parser for Char {
    type Output = char;

    fn parse_null(&self, forest: &mut Forest<Self::Output>) {
        forest.add(self.0);
    }

    fn derivative(&self, character: char) -> Box<dyn Parser<Output = Self::Output>> {
        if character == self.0 {
            Box::new(Null(self.0))
        } else {
            Box::<Void<char>>::default()
        }
    }

    fn clone_box(&self) -> Box<dyn Parser<Output = Self::Output>> {
        Box::new(*self)
    }
}

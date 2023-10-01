use crate::{
    combinators::{choice::Choice, is_null::IsNull},
    forest::Forest,
    parser::{CanParse, Parser},
};
use std::hash::Hash;

pub struct Then<T, U> {
    first: Box<dyn Parser<Output = T>>,
    second: Box<dyn Parser<Output = U>>,
}

impl<T: Eq + Hash + Clone + 'static, U: Eq + Hash + Clone + 'static> Parser for Then<T, U> {
    type Output = (T, U);

    fn parse_null(&self, forest: &mut Forest<Self::Output>) {
        let mut first_forest = Forest::default();
        let mut second_forest = Forest::default();

        self.first.parse_null(&mut first_forest);
        self.second.parse_null(&mut second_forest);

        forest.extend(Forest::tuple_product(first_forest, second_forest));
    }

    fn derivative(&self, character: char) -> Box<dyn Parser<Output = Self::Output>> {
        let choice = Choice::new(vec![
            Box::new(Then::new(
                self.first.derivative(character),
                self.second.clone_box(),
            )),
            Box::new(Then::new(
                IsNull::new(self.first.as_ref()),
                self.second.derivative(character),
            )),
        ]);

        Box::new(choice)
    }

    fn clone_box(&self) -> Box<dyn Parser<Output = Self::Output>> {
        Box::new(Self {
            first: self.first.clone_box(),
            second: self.second.clone_box(),
        })
    }
}

impl<T: Eq + Hash + Clone + 'static, U: Eq + Hash + Clone + 'static> Then<T, U> {
    pub fn new(first: impl CanParse<Output = T>, second: impl CanParse<Output = U>) -> Self {
        Self {
            first: first.parser(),
            second: second.parser(),
        }
    }
}

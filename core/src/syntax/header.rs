use pest::Parser;
use pest_derive::Parser;

use crate::{Error, model::Proto, parser::ProtoParser};

#[derive(Debug, Default, Clone, Copy, Parser)]
#[grammar = "./grammar/literal.pest"]
#[grammar = "./grammar/syntax.pest"]
pub(crate) struct InternalParser;

impl<T> ProtoParser for T
where
    T: IntoIterator<Item = Box<dyn ProtoParser>>,
{
    fn parse<'a>(&self, input: &'a str) -> Result<Proto<'a>, Error> {
        let pairs = InternalParser::parse(Rule::proto, input)?;
        for pair in pairs {
            println!("{pair:?}");
        }
        todo!()
    }
}

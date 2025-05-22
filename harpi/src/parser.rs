use crate::{Error, model::Syntax, visitor::ProtoVisitor};

pub trait ProtoParser {
    const SYNTAX: &'static str;
    fn parse<'a, Visitor>(data: &'a str, visitor: &mut Visitor) -> Result<(), Error>
    where
        Visitor: ProtoVisitor;

    fn parse_with_syntax<'a, Visitor>(
        data: &'a str,
        syntax: Syntax<'a>,
        visitor: &mut Visitor,
    ) -> Result<(), Error>
    where
        Visitor: ProtoVisitor;
}

use crate::{Error, model::Proto};

pub trait ProtoSyntaxResolver {
    fn resolve_syntax<'a>(&self, syntax: &str) -> Option<Box<dyn ProtoSyntax>>;
}
pub trait ProtoParser {
    fn parse<'a>(&self, input: &'a str) -> Result<Proto<'a>, Error>;
}
pub trait ProtoSyntax {
    fn syntax<'a>(&self) -> &'a str;
    fn parse<'a>(&self, data: &'a str) -> Result<Proto<'a>, Error>;
}

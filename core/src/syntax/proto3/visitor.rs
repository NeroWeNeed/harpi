use crate::model::{Enum, Import, Message, Package, Service, Syntax};

#[derive(Debug, Clone)]
pub enum Proto3Node<'a> {
    Start,
    Syntax(Syntax<'a>),
    Package(Package<'a>),
    Import(Import<'a>),
    Option(crate::model::Option<'a>),
    Message(Message<'a>),
    Service(Service<'a>),
    Enum(Enum<'a>),
    End,
}
pub trait ProtoVisitor {
    type Error;
    fn on<'a>(&mut self, node: Proto3Node<'a>) -> Result<(), Self::Error>;
}

use crate::model::{Enum, Import, Message, Package, Service, Syntax};

#[derive(Debug, Clone)]
pub enum Node<'a> {
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
    fn on<'a>(&mut self, node: Node<'a>);
}

use std::{
    borrow::Cow,
    fmt::{Display, Write},
};

use builder::Builder;
use getter::Getter;
#[derive(Debug, Clone, Builder, Getter)]
pub struct Proto<'a> {
    syntax: Syntax<'a>,
    package: Package<'a>,
    #[builder(setter_name = "import")]
    imports: Cow<'a, [Import<'a>]>,
    #[builder(setter_name = "option")]
    options: Cow<'a, [Option<'a>]>,
    #[builder(setter_name = "message")]
    messages: Cow<'a, [Message<'a>]>,
    #[builder(setter_name = "enum")]
    enums: Cow<'a, [Enum<'a>]>,
    #[builder(setter_name = "service")]
    services: Cow<'a, [Service<'a>]>,
}
#[derive(Debug, Clone, Default, Builder, Getter)]
pub struct Package<'a> {
    #[builder(into)]
    value: Cow<'a, str>,
    #[builder(setter_name = "comment")]
    comments: Cow<'a, [Comment<'a>]>,
}
#[derive(Debug, Clone, Default, Builder, Getter)]
pub struct Syntax<'a> {
    #[builder(into)]
    value: Cow<'a, str>,
    #[builder(setter_name = "comment")]
    comments: Cow<'a, [Comment<'a>]>,
}
#[derive(Debug, Clone, Default, Builder, Getter)]
pub struct Import<'a> {
    weak: bool,
    public: bool,
    #[builder(into)]
    value: Cow<'a, str>,
    #[builder(setter_name = "comment")]
    comments: Cow<'a, [Comment<'a>]>,
}
#[derive(Debug, Clone, Builder, Getter)]
#[builder(required)]
pub struct Option<'a> {
    name: Ident<'a>,
    value: Constant<'a>,
    #[builder(setter_name = "comment")]
    comments: Cow<'a, [Comment<'a>]>,
}
#[derive(Debug, Clone, Builder, Getter)]
pub struct Service<'a> {
    name: Ident<'a>,
    #[builder(setter_name = "rpc")]
    rpcs: Cow<'a, [ServiceRpc<'a>]>,
    #[builder(setter_name = "option")]
    options: Cow<'a, [super::Option<'a>]>,
    #[builder(setter_name = "comment")]
    comments: Cow<'a, [Comment<'a>]>,
}
#[derive(Debug, Clone, Builder, Getter)]
#[builder(required)]
pub struct ServiceRpc<'a> {
    name: Ident<'a>,
    input: ServiceRpcField<'a>,
    output: ServiceRpcField<'a>,
    #[builder(setter_name = "option")]
    options: Cow<'a, [super::Option<'a>]>,
    #[builder(setter_name = "comment")]
    comments: Cow<'a, [Comment<'a>]>,
}
#[derive(Debug, Clone, Builder, Getter)]
#[builder(required)]
pub struct ServiceRpcField<'a> {
    value: MessageReference<'a>,
    #[builder(optional)]
    stream: bool,
}
#[derive(Debug, Clone, Builder, Getter)]
#[builder(required)]
pub struct Message<'a> {
    name: Ident<'a>,
    #[builder(setter_name = "field")]
    fields: Cow<'a, [Field<'a>]>,
    #[builder(setter_name = "enum")]
    enums: Cow<'a, [Enum<'a>]>,
    #[builder(setter_name = "message")]
    messages: Cow<'a, [Message<'a>]>,
    #[builder(setter_name = "option")]
    options: Cow<'a, [super::Option<'a>]>,
    #[builder(setter_name = "reserved")]
    reserved: Cow<'a, [ReservedItems<'a>]>,
    #[builder(setter_name = "comment")]
    comments: Cow<'a, [Comment<'a>]>,
}
#[derive(Debug, Clone, Builder, Getter)]
pub struct Enum<'a> {
    name: Ident<'a>,
    #[builder(setter_name = "field")]
    fields: Cow<'a, [EnumItem<'a>]>,
    #[builder(setter_name = "reserved")]
    reserved: Cow<'a, [ReservedItems<'a>]>,
    #[builder(setter_name = "option")]
    options: Cow<'a, [Option<'a>]>,
    #[builder(setter_name = "comment")]
    comments: Cow<'a, [Comment<'a>]>,
}

#[derive(Debug, Clone, Builder, Getter)]
#[builder(required)]
pub struct ReservedItems<'a> {
    #[builder(setter_name = "item")]
    items: Cow<'a, [ReservedData<'a>]>,
    #[builder(setter_name = "comment")]
    comments: Cow<'a, [Comment<'a>]>,
}
#[derive(Debug, Clone)]
pub enum ReservedData<'a> {
    Range(i64, i64),
    Field(Ident<'a>),
}

#[derive(Debug, Clone, Builder, Getter)]
pub struct EnumItem<'a> {
    name: Ident<'a>,
    number: i64,
    #[builder(setter_name = "option")]
    options: Cow<'a, [Option<'a>]>,
    #[builder(setter_name = "comment")]
    comments: Cow<'a, [Comment<'a>]>,
}

#[derive(Debug, Clone)]
pub enum Field<'a> {
    Normal(NormalField<'a>),
    OneOf(OneOfField<'a>),
    Map(MapField<'a>),
}
#[derive(Debug, Clone, Builder, Getter)]
#[builder(required)]
pub struct NormalField<'a> {
    #[builder(optional)]
    repeated: bool,
    #[builder(optional)]
    optional: bool,
    ty: Type<'a>,
    name: Ident<'a>,
    number: u64,
    #[builder(setter_name = "option")]
    options: Cow<'a, [super::Option<'a>]>,
    #[builder(setter_name = "comment")]
    comments: Cow<'a, [Comment<'a>]>,
}

#[derive(Debug, Clone, Builder, Getter)]
pub struct OneOfField<'a> {
    name: Ident<'a>,
    #[builder(setter_name = "field")]
    fields: Cow<'a, [OneOfFieldItem<'a>]>,
    #[builder(setter_name = "option")]
    options: Cow<'a, [super::Option<'a>]>,
    #[builder(setter_name = "comment")]
    comments: Cow<'a, [Comment<'a>]>,
}

#[derive(Debug, Clone, Builder, Getter)]
#[builder(required)]
pub struct OneOfFieldItem<'a> {
    ty: Type<'a>,
    name: Ident<'a>,
    number: u64,
    #[builder(setter_name = "option")]
    options: Cow<'a, [super::Option<'a>]>,
    #[builder(setter_name = "comment")]
    comments: Cow<'a, [Comment<'a>]>,
}

#[derive(Debug, Clone, Builder, Getter)]
#[builder(required)]
pub struct MapField<'a> {
    pub(super) key_ty: MapFieldKeyType,
    pub(super) value_ty: Type<'a>,
    pub(super) name: Ident<'a>,
    pub(super) number: u64,
    #[builder(setter_name = "option")]
    pub(super) options: Cow<'a, [super::Option<'a>]>,
    #[builder(setter_name = "comment")]
    pub(super) comments: Cow<'a, [Comment<'a>]>,
}
#[derive(Debug, Clone, Copy)]
pub enum MapFieldKeyType {
    Int32,
    Int64,
    UInt32,
    UInt64,
    SInt32,
    SInt64,
    Fixed32,
    Fixed64,
    SFixed32,
    SFixed64,
    Bool,
    String,
}
#[derive(Debug, Clone)]
pub enum Constant<'a> {
    Ident(Ident<'a>),
    Int(i64),
    Float(f64),
    String(Cow<'a, str>),
    Bool(bool),
}
#[derive(Debug, Clone)]
pub enum Type<'a> {
    Double,
    Float,
    Int32,
    Int64,
    UInt32,
    UInt64,
    SInt32,
    SInt64,
    Fixed32,
    Fixed64,
    SFixed32,
    SFixed64,
    Bool,
    String,
    Bytes,
    Reference(Cow<'a, str>),
}
#[derive(Debug, Clone, Getter)]
pub struct MessageReference<'a>(#[getter(name = "value")] Cow<'a, str>);

impl<'a> MessageReference<'a> {
    pub fn new(value: impl Into<Cow<'a, str>>) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Ident<'a> {
    relative: bool,
    value: Cow<'a, str>,
}
#[derive(Debug, Clone, Getter)]
pub struct Comment<'a>(#[getter(name = "value")] Cow<'a, str>);
impl<'a> Comment<'a> {
    pub fn new(value: impl Into<Cow<'a, str>>) -> Self {
        Comment(value.into())
    }
}

impl<'a> Ident<'a> {
    pub fn new(relative: bool, value: impl Into<Cow<'a, str>>) -> Self {
        Self {
            relative,
            value: value.into(),
        }
    }

    pub fn relative(&self) -> bool {
        self.relative
    }

    pub fn set_relative(&mut self, relative: bool) {
        self.relative = relative;
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn set_value(&mut self, value: impl Into<Cow<'a, str>>) {
        self.value = value.into();
    }
}
impl<'a> Display for Ident<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.relative {
            f.write_char('.')?;
        }
        f.write_str(self.value.as_ref())
    }
}

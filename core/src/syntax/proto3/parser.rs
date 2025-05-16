use pest::{
    Parser,
    iterators::{Pair, Pairs},
};

use crate::{
    Error, ProtoParser,
    model::{
        self, Comment, Constant, Enum, EnumBuilder, EnumItem, Ident, Import, MapField,
        MapFieldKeyType, Message, MessageBuilder, MessageReference, NormalField, OneOfField,
        OneOfFieldBuilder, OneOfFieldItem, Package, Proto, ReservedData, ReservedItems,
        ReservedItemsBuilder, Service, ServiceBuilder, ServiceRpc, ServiceRpcField, Syntax, Type,
    },
    parser::ProtoSyntax,
    proto3::{
        parse_bool, parse_ident, parse_literal_int, parse_literal_signed_int, parse_literal_string,
        parse_literal_unsigned_int, parse_signed_float,
    },
};

#[derive(Debug, Default, Clone, Copy, pest_derive::Parser)]
#[grammar = "./grammar/literal.pest"]
#[grammar = "./grammar/proto.pest"]
pub(crate) struct InternalParser;
#[derive(Debug, Clone, Copy, Default)]
pub struct Proto3;

type Proto3Rule = Rule;
type Proto3Pairs<'a> = Pairs<'a, Proto3Rule>;
type Proto3Pair<'a> = Pair<'a, Proto3Rule>;
type Proto3Result<T> = Result<T, Error>;

const SYNTAX: &'static str = "proto3";
impl ProtoParser for Proto3 {
    fn parse<'a>(&self, input: &'a str) -> Proto3Result<crate::model::Proto<'a>> {
        let pairs = InternalParser::parse(Rule::proto_with_syntax, input)?;
        let mut result = Err(Error::UndefinedParsingRoute);
        for pair in pairs {
            let rule = pair.as_rule();
            match rule {
                Rule::proto_with_syntax => {
                    result = parse(pair.into_inner());
                    break;
                }
                _ => {
                    result = Err(Error::UndefinedParsingRoute);
                    break;
                }
            }
        }
        result
    }
}
impl ProtoSyntax for Proto3 {
    fn parse<'a>(&self, data: &'a str) -> Proto3Result<crate::model::Proto<'a>> {
        let pairs = InternalParser::parse(Rule::proto, data)?;
        let mut result = Err(Error::UndefinedParsingRoute);
        for pair in pairs {
            let rule = pair.as_rule();
            match rule {
                Rule::proto => {
                    result = parse(pair.into_inner());
                    break;
                }
                _ => {
                    result = Err(Error::UndefinedParsingRoute);
                    break;
                }
            }
        }
        result
    }

    fn syntax<'a>(&self) -> &'a str {
        SYNTAX
    }
}
fn parse<'a>(pairs: Proto3Pairs<'a>) -> Proto3Result<crate::model::Proto<'a>> {
    let mut proto = Proto::builder();
    for pair in pairs {
        let rule = pair.as_rule();
        match rule {
            Rule::EOI => break,
            Rule::syntax => {
                proto.set_syntax(parse_syntax(pair)?);
            }
            Rule::import => {
                proto.with_import(parse_import(pair)?);
            }
            Rule::package => {
                proto.set_package(parse_package(pair)?);
            }
            Rule::option => {
                proto.with_option(parse_option(pair)?);
            }
            Rule::r#enum => {
                proto.with_enum(parse_enum(pair)?);
            }
            Rule::message => {
                proto.with_message(parse_message(pair)?);
            }
            Rule::service => {
                proto.with_service(parse_service(pair)?);
            }
            _ => return Err(Error::UndefinedParsingRoute),
        }
    }
    Ok(proto.build())
}
fn parse_syntax<'a>(pair: Proto3Pair<'a>) -> Proto3Result<Syntax<'a>> {
    let pairs = pair.into_inner();
    let mut builder = Syntax::builder();
    for pair in pairs {
        let rule = pair.as_rule();
        match rule {
            Rule::COMMENT => {
                builder.with_comment(parse_comment(pair)?);
            }
            Rule::syntax_proto3 => {
                builder.set_value(SYNTAX);
            }

            _ => return Err(Error::UndefinedParsingRoute),
        }
    }

    Ok(builder.build())
}
fn parse_import<'a>(pair: Proto3Pair<'a>) -> Proto3Result<Import<'a>> {
    let pairs = pair.into_inner();
    let mut builder = Import::builder();
    for pair in pairs {
        let rule = pair.as_rule();
        match rule {
            Rule::keyword_weak => {
                builder.set_weak(true);
            }
            Rule::keyword_public => {
                builder.set_public(true);
            }
            Rule::STRING_LIT => {
                let output = parse_literal_string(pair)?;
                println!("Import: {:?}", output);
                builder.set_value(output);
            }
            Rule::COMMENT => {
                builder.with_comment(parse_comment(pair)?);
            }

            _ => return Err(Error::UndefinedParsingRoute),
        }
    }

    Ok(builder.build())
}

fn parse_package<'a>(pair: Proto3Pair<'a>) -> Proto3Result<Package<'a>> {
    let pairs = pair.into_inner();
    let mut builder = Package::builder();
    for pair in pairs {
        let rule = pair.as_rule();
        match rule {
            Rule::COMMENT => {
                builder.with_comment(parse_comment(pair)?);
            }
            Rule::FULL_IDENT => {
                builder.set_value(pair.as_str());
            }
            _ => {
                return Err(Error::UndefinedParsingRoute);
            }
        }
    }
    Ok(builder.build())
}
fn parse_option<'a>(pair: Proto3Pair<'a>) -> Proto3Result<crate::model::Option<'a>> {
    let pairs = pair.into_inner();
    let mut builder = model::Option::builder();
    for pair in pairs {
        let rule = pair.as_rule();
        match rule {
            Rule::option_name => {
                builder.set_name(parse_ident(pair)?);
            }
            Rule::CONSTANT => {
                builder.set_value(parse_constant(pair)?);
            }
            Rule::COMMENT => {
                builder.with_comment(parse_comment(pair)?);
            }
            _ => {
                return Err(Error::UndefinedParsingRoute);
            }
        }
    }
    Ok(builder.build()?)
}

fn parse_reserved<'a>(pair: Proto3Pair<'a>) -> Proto3Result<ReservedItems<'a>> {
    fn parse_reserved_ranges<'a>(
        pair: Proto3Pair<'a>,
        builder: &mut ReservedItemsBuilder<'a>,
    ) -> Proto3Result<()> {
        fn parse_reserved_range<'a>(pair: Proto3Pair<'a>) -> Proto3Result<ReservedData<'a>> {
            let mut value = None;
            let pairs = pair.into_inner();
            for pair in pairs {
                let rule = pair.as_rule();
                let range_value = match rule {
                    Rule::INT_LIT => parse_literal_int(pair),
                    Rule::keyword_max => Ok(i64::MAX),
                    _ => Err(Error::UndefinedParsingRoute),
                }?;
                value = if let Some((old, _)) = value {
                    Some((old, range_value))
                } else {
                    Some((range_value, range_value))
                };
            }
            value
                .ok_or(Error::UndefinedParsingRoute)
                .map(|(start, end)| ReservedData::Range(start, end))
        }
        let pairs = pair.into_inner();
        for pair in pairs {
            let rule = pair.as_rule();
            match rule {
                Rule::range => {
                    builder.with_item(parse_reserved_range(pair)?);
                }
                _ => {
                    return Err(Error::UndefinedParsingRoute);
                }
            }
        }
        Ok(())
    }
    fn parse_reserved_fields<'a>(
        pair: Proto3Pair<'a>,
        builder: &mut ReservedItemsBuilder<'a>,
    ) -> Proto3Result<()> {
        let pairs = pair.into_inner();
        for pair in pairs {
            let rule = pair.as_rule();
            match rule {
                Rule::str_field_name => {
                    builder.with_item(ReservedData::Field(Ident::new(false, pair.as_str())));
                }
                _ => {
                    return Err(Error::UndefinedParsingRoute);
                }
            }
        }
        Ok(())
    }
    let pairs = pair.into_inner();
    let mut builder = ReservedItems::builder();
    for pair in pairs {
        let rule = pair.as_rule();
        match rule {
            Rule::ranges => {
                parse_reserved_ranges(pair, &mut builder)?;
            }
            Rule::str_field_names => {
                parse_reserved_fields(pair, &mut builder)?;
            }
            Rule::COMMENT => {
                builder.with_comment(parse_comment(pair)?);
            }

            _ => {
                return Err(Error::UndefinedParsingRoute);
            }
        }
    }
    Ok(builder.build())
}

pub(super) fn parse_constant<'a>(pair: Proto3Pair<'a>) -> Proto3Result<Constant<'a>> {
    let pairs = pair.into_inner();
    for pair in pairs {
        let rule = pair.as_rule();
        match rule {
            Rule::FULL_IDENT => {
                return Ok(Constant::Ident(Ident::new(false, pair.as_str())));
            }
            Rule::SIGNED_INT_LIT => {
                return Ok(Constant::Int(parse_literal_signed_int(pair)?));
            }
            Rule::SIGNED_FLOAT_LIT => {
                return Ok(Constant::Float(parse_signed_float(pair)?));
            }
            Rule::BOOL_LIT => {
                return Ok(Constant::Bool(parse_bool(pair)?));
            }
            Rule::STRING_LIT => {
                return Ok(Constant::String(parse_literal_string(pair)?));
            }
            _ => {
                return Err(Error::UndefinedParsingRoute);
            }
        }
    }
    Err(Error::UndefinedParsingRoute)
}
fn parse_enum<'a>(pair: Proto3Pair<'a>) -> Proto3Result<Enum<'a>> {
    fn parse_enum_body<'b>(
        builder: &mut EnumBuilder<'b>,
        pair: Proto3Pair<'b>,
    ) -> Proto3Result<()> {
        fn parse_enum_field<'c>(pair: Proto3Pair<'c>) -> Proto3Result<EnumItem<'c>> {
            let pairs = pair.into_inner();
            let mut builder = EnumItem::builder();
            for pair in pairs {
                let rule = pair.as_rule();

                match rule {
                    Rule::IDENT => {
                        builder.set_name(Ident::new(false, pair.as_str()));
                    }
                    Rule::SIGNED_INT_LIT => {
                        builder.set_number(parse_literal_signed_int(pair)?);
                    }
                    Rule::enum_value_option => {
                        builder.with_option(parse_option(pair)?);
                    }
                    Rule::COMMENT => {
                        builder.with_comment(parse_comment(pair)?);
                    }
                    _ => {
                        return Err(Error::UndefinedParsingRoute);
                    }
                }
            }
            Ok(builder.build())
        }
        let pairs = pair.into_inner();
        for pair in pairs {
            let rule = pair.as_rule();

            match rule {
                Rule::option => {
                    builder.with_option(parse_option(pair)?);
                }
                Rule::enum_field => {
                    builder.with_field(parse_enum_field(pair)?);
                }
                Rule::reserved => {
                    builder.with_reserved(parse_reserved(pair)?);
                }
                Rule::COMMENT => {
                    builder.with_comment(parse_comment(pair)?);
                }
                _ => {
                    return Err(Error::UndefinedParsingRoute);
                }
            }
        }
        Ok(())
    }

    let pairs = pair.into_inner();
    let mut builder = Enum::builder();
    for pair in pairs {
        let rule = pair.as_rule();
        match rule {
            Rule::enum_name => {
                builder.set_name(Ident::new(false, pair.as_str()));
            }
            Rule::enum_body => {
                parse_enum_body(&mut builder, pair)?;
            }
            Rule::COMMENT => {
                builder.with_comment(parse_comment(pair)?);
            }
            _ => {
                return Err(Error::UndefinedParsingRoute);
            }
        }
    }
    Ok(builder.build())
}
fn parse_type<'a>(pair: Proto3Pair<'a>) -> Proto3Result<Type<'a>> {
    let pairs = pair.into_inner();
    for pair in pairs {
        let rule = pair.as_rule();
        match rule {
            Rule::keyword_double => {
                return Ok(Type::Double);
            }
            Rule::keyword_float => {
                return Ok(Type::Float);
            }
            Rule::keyword_int32 => {
                return Ok(Type::Int32);
            }
            Rule::keyword_int64 => {
                return Ok(Type::Int64);
            }
            Rule::keyword_uint32 => {
                return Ok(Type::UInt32);
            }
            Rule::keyword_uint64 => {
                return Ok(Type::UInt64);
            }
            Rule::keyword_sint32 => {
                return Ok(Type::SInt32);
            }
            Rule::keyword_sint64 => {
                return Ok(Type::SInt64);
            }
            Rule::keyword_fixed32 => {
                return Ok(Type::Fixed32);
            }
            Rule::keyword_fixed64 => {
                return Ok(Type::Fixed64);
            }
            Rule::keyword_sfixed32 => {
                return Ok(Type::SFixed32);
            }
            Rule::keyword_sfixed64 => {
                return Ok(Type::SFixed64);
            }
            Rule::keyword_bool => {
                return Ok(Type::Bool);
            }
            Rule::keyword_string => {
                return Ok(Type::String);
            }
            Rule::keyword_bytes => {
                return Ok(Type::Bytes);
            }
            Rule::user_type => {
                return Ok(Type::Reference(pair.into_inner().as_str().into()));
            }
            _ => {
                return Err(Error::UndefinedParsingRoute);
            }
        }
    }
    Err(Error::UndefinedParsingRoute)
}
fn parse_map_type<'a>(pair: Proto3Pair<'a>) -> Proto3Result<MapFieldKeyType> {
    let pairs = pair.into_inner();
    for pair in pairs {
        let rule = pair.as_rule();
        match rule {
            Rule::keyword_int32 => {
                return Ok(MapFieldKeyType::Int32);
            }
            Rule::keyword_int64 => {
                return Ok(MapFieldKeyType::Int64);
            }
            Rule::keyword_uint32 => {
                return Ok(MapFieldKeyType::UInt32);
            }
            Rule::keyword_uint64 => {
                return Ok(MapFieldKeyType::UInt64);
            }
            Rule::keyword_sint32 => {
                return Ok(MapFieldKeyType::SInt32);
            }
            Rule::keyword_sint64 => {
                return Ok(MapFieldKeyType::SInt64);
            }
            Rule::keyword_fixed32 => {
                return Ok(MapFieldKeyType::Fixed32);
            }
            Rule::keyword_fixed64 => {
                return Ok(MapFieldKeyType::Fixed64);
            }
            Rule::keyword_sfixed32 => {
                return Ok(MapFieldKeyType::SFixed32);
            }
            Rule::keyword_sfixed64 => {
                return Ok(MapFieldKeyType::SFixed64);
            }
            Rule::keyword_bool => {
                return Ok(MapFieldKeyType::Bool);
            }
            Rule::keyword_string => {
                return Ok(MapFieldKeyType::String);
            }
            _ => {
                return Err(Error::UndefinedParsingRoute);
            }
        }
    }
    Err(Error::UndefinedParsingRoute)
}

fn parse_message<'a>(pair: Proto3Pair<'a>) -> Proto3Result<Message<'a>> {
    fn parse_message_body<'b>(
        builder: &mut MessageBuilder<'b>,
        pair: Proto3Pair<'b>,
    ) -> Proto3Result<()> {
        fn parse_normal_field<'c>(pair: Proto3Pair<'c>) -> Proto3Result<NormalField<'c>> {
            let pairs = pair.into_inner();
            let mut builder = NormalField::builder();
            for pair in pairs {
                let rule = pair.as_rule();
                match rule {
                    Rule::keyword_repeated => {
                        builder.set_repeated(true);
                    }
                    Rule::keyword_optional => {
                        builder.set_optional(true);
                    }
                    Rule::r#type => {
                        builder.set_ty(parse_type(pair)?);
                    }
                    Rule::IDENT => {
                        builder.set_name(Ident::new(false, pair.as_str()));
                    }
                    Rule::INT_LIT => {
                        builder.set_number(parse_literal_unsigned_int(pair)?);
                    }
                    Rule::field_option => {
                        builder.with_option(parse_option(pair)?);
                    }
                    Rule::COMMENT => {
                        builder.with_comment(parse_comment(pair)?);
                    }
                    _ => {
                        return Err(Error::UndefinedParsingRoute);
                    }
                }
            }
            Ok(builder.build()?)
        }
        fn parse_one_of_field<'c>(pair: Proto3Pair<'c>) -> Proto3Result<OneOfField<'c>> {
            fn parse_one_of_body<'c>(
                builder: &mut OneOfFieldBuilder<'c>,
                pair: Proto3Pair<'c>,
            ) -> Proto3Result<()> {
                fn parse_one_of_field_item<'c>(
                    pair: Proto3Pair<'c>,
                ) -> Proto3Result<OneOfFieldItem<'c>> {
                    let pairs = pair.into_inner();
                    let mut builder = OneOfFieldItem::builder();
                    for pair in pairs {
                        let rule = pair.as_rule();
                        match rule {
                            Rule::r#type => {
                                builder.set_ty(parse_type(pair)?);
                            }
                            Rule::IDENT => {
                                builder.set_name(Ident::new(false, pair.as_str()));
                            }
                            Rule::INT_LIT => {
                                builder.set_number(parse_literal_unsigned_int(pair)?);
                            }
                            Rule::field_option => {
                                builder.with_option(parse_option(pair)?);
                            }
                            Rule::COMMENT => {
                                builder.with_comment(parse_comment(pair)?);
                            }
                            _ => {
                                return Err(Error::UndefinedParsingRoute);
                            }
                        }
                    }
                    Ok(builder.build()?)
                }
                let pairs = pair.into_inner();
                for pair in pairs {
                    let rule = pair.as_rule();
                    match rule {
                        Rule::one_of_field => {
                            builder.with_field(parse_one_of_field_item(pair)?);
                        }
                        Rule::option => {
                            builder.with_option(parse_option(pair)?);
                        }
                        Rule::COMMENT => {
                            builder.with_comment(parse_comment(pair)?);
                        }
                        _ => {
                            return Err(Error::UndefinedParsingRoute);
                        }
                    }
                }
                Ok(())
            }

            let pairs = pair.into_inner();
            let mut builder = OneOfField::builder();
            for pair in pairs {
                let rule = pair.as_rule();
                match rule {
                    Rule::IDENT => {
                        builder.set_name(Ident::new(false, pair.as_str()));
                    }
                    Rule::one_of_body => {
                        parse_one_of_body(&mut builder, pair)?;
                    }
                    Rule::COMMENT => {
                        builder.with_comment(parse_comment(pair)?);
                    }
                    _ => {
                        return Err(Error::UndefinedParsingRoute);
                    }
                }
            }
            Ok(builder.build())
        }
        fn parse_map_field<'c>(pair: Proto3Pair<'c>) -> Proto3Result<MapField<'c>> {
            let pairs = pair.into_inner();
            let mut builder = MapField::builder();
            for pair in pairs {
                let rule = pair.as_rule();
                match rule {
                    Rule::key_type => {
                        builder.set_key_ty(parse_map_type(pair)?);
                    }

                    Rule::r#type => {
                        builder.set_value_ty(parse_type(pair)?);
                    }
                    Rule::IDENT => {
                        builder.set_name(Ident::new(false, pair.as_str()));
                    }
                    Rule::INT_LIT => {
                        builder.set_number(parse_literal_unsigned_int(pair)?);
                    }
                    Rule::field_option => {
                        builder.with_option(parse_option(pair)?);
                    }
                    Rule::COMMENT => {
                        builder.with_comment(parse_comment(pair)?);
                    }
                    _ => {
                        return Err(Error::UndefinedParsingRoute);
                    }
                }
            }
            Ok(builder.build()?)
        }
        let pairs = pair.into_inner();
        for pair in pairs {
            let rule = pair.as_rule();
            match rule {
                Rule::option => {
                    builder.with_option(parse_option(pair)?);
                }
                Rule::message => {
                    builder.with_message(parse_message(pair)?);
                }
                Rule::r#enum => {
                    builder.with_enum(parse_enum(pair)?);
                }
                Rule::reserved => {
                    builder.with_reserved(parse_reserved(pair)?);
                }
                Rule::field => {
                    builder.with_field(model::Field::Normal(parse_normal_field(pair)?));
                }
                Rule::one_of => {
                    builder.with_field(model::Field::OneOf(parse_one_of_field(pair)?));
                }
                Rule::map_field => {
                    builder.with_field(model::Field::Map(parse_map_field(pair)?));
                }
                Rule::COMMENT => {
                    builder.with_comment(parse_comment(pair)?);
                }
                _ => {
                    return Err(Error::UndefinedParsingRoute);
                }
            }
        }
        Ok(())
    }

    let pairs = pair.into_inner();
    let mut builder = Message::builder();
    for pair in pairs {
        let rule = pair.as_rule();
        match rule {
            Rule::message_name => {
                builder.set_name(Ident::new(false, pair.as_str()));
            }
            Rule::message_body => {
                parse_message_body(&mut builder, pair)?;
            }
            Rule::COMMENT => {
                builder.with_comment(parse_comment(pair)?);
            }
            _ => {
                return Err(Error::UndefinedParsingRoute);
            }
        }
    }
    Ok(builder.build()?)
}
fn parse_service<'a>(pair: Proto3Pair<'a>) -> Proto3Result<Service<'a>> {
    fn parse_service_body<'b>(
        builder: &mut ServiceBuilder<'b>,
        pair: Proto3Pair<'b>,
    ) -> Proto3Result<()> {
        fn parse_service_rpc<'c>(pair: Proto3Pair<'c>) -> Proto3Result<ServiceRpc<'c>> {
            fn parse_service_rpc_field<'d>(
                pair: Proto3Pair<'d>,
            ) -> Proto3Result<ServiceRpcField<'d>> {
                let pairs = pair.into_inner();
                let mut builder = ServiceRpcField::builder();
                for pair in pairs {
                    let rule = pair.as_rule();
                    match rule {
                        Rule::keyword_stream => {
                            builder.set_stream(true);
                        }
                        Rule::message_type => {
                            builder.set_value(MessageReference::new(pair.as_str()));
                        }

                        _ => {
                            return Err(Error::UndefinedParsingRoute);
                        }
                    }
                }
                builder.build().map_err(|_| Error::UndefinedParsingRoute)
            }
            let pairs = pair.into_inner();
            let mut builder = ServiceRpc::builder();
            for pair in pairs {
                let rule = pair.as_rule();

                match rule {
                    Rule::rpc_name => {
                        builder.set_name(Ident::new(false, pair.as_str()));
                    }
                    Rule::rpc_input => {
                        builder.set_input(parse_inner(
                            pair,
                            Rule::rpc_field,
                            parse_service_rpc_field,
                        )?);
                    }
                    Rule::rpc_output => {
                        builder.set_output(parse_inner(
                            pair,
                            Rule::rpc_field,
                            parse_service_rpc_field,
                        )?);
                    }
                    Rule::option => {
                        builder.with_option(parse_option(pair)?);
                    }
                    Rule::COMMENT => {
                        builder.with_comment(parse_comment(pair)?);
                    }
                    _ => {
                        return Err(Error::UndefinedParsingRoute);
                    }
                }
            }
            builder.build().map_err(|_| Error::UndefinedParsingRoute)
        }
        let pairs = pair.into_inner();
        for pair in pairs {
            let rule = pair.as_rule();
            match rule {
                Rule::option => {
                    builder.with_option(parse_option(pair)?);
                }
                Rule::rpc => {
                    builder.with_rpc(parse_service_rpc(pair)?);
                }
                _ => {
                    return Err(Error::UndefinedParsingRoute);
                }
            }
        }
        Ok(())
    }
    let pairs = pair.into_inner();
    let mut builder = Service::builder();
    for pair in pairs {
        let rule = pair.as_rule();

        match rule {
            Rule::COMMENT => {
                builder.with_comment(parse_comment(pair)?);
            }
            Rule::service_name => {
                builder.set_name(Ident::new(false, pair.as_str()));
            }
            Rule::service_body => {
                parse_service_body(&mut builder, pair)?;
            }

            _ => {
                return Err(Error::UndefinedParsingRoute);
            }
        }
    }
    Ok(builder.build())
}
fn parse_inner<'a, F, R>(pair: Proto3Pair<'a>, rule: Proto3Rule, handle: F) -> Proto3Result<R>
where
    F: FnOnce(Proto3Pair<'a>) -> Proto3Result<R>,
{
    let pair = pair
        .into_inner()
        .next()
        .ok_or(Error::UndefinedParsingRoute)?;
    if pair.as_rule() == rule {
        handle(pair)
    } else {
        Err(Error::UndefinedParsingRoute)
    }
}
fn parse_comment<'a>(pair: Proto3Pair<'a>) -> Proto3Result<Comment<'a>> {
    let pairs = pair.into_inner();
    for pair in pairs {
        let rule = pair.as_rule();
        match rule {
            Rule::COMMENT_LINE => {
                return parse_inner(pair, Rule::COMMENT_LINE_INNER, |pair| {
                    Ok(Comment::new(pair.as_str()))
                });
            }
            Rule::COMMENT_BLOCK => {
                return parse_inner(pair, Rule::COMMENT_BLOCK_INNER, |pair| {
                    Ok(Comment::new(pair.as_str()))
                });
            }

            _ => {
                return Err(Error::UndefinedParsingRoute);
            }
        }
    }
    Err(Error::UndefinedParsingRoute)
}

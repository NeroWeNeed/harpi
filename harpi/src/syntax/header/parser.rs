use pest::{
    Parser,
    iterators::{Pair, Pairs},
};
use pest_derive::Parser;

use crate::{
    Error,
    model::{Comment, Syntax},
};

use super::literals::parse_literal_string;

#[derive(Debug, Default, Clone, Copy, Parser)]
#[grammar = "./grammar/literal.pest"]
#[grammar = "./grammar/syntax.pest"]
pub(crate) struct InternalParser;

type HeaderRule = Rule;
type HeaderPairs<'a> = Pairs<'a, HeaderRule>;
type HeaderPair<'a> = Pair<'a, HeaderRule>;
type HeaderResult<T> = Result<T, Error>;

pub fn parse_header<'a>(input: &'a str) -> Result<(Syntax<'a>, &'a str), Error> {
    let pairs = InternalParser::parse(Rule::proto, input)?;
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
    result.map(|(syntax, offset)| (syntax, input.split_at(offset).1))
}

fn parse<'a>(pairs: HeaderPairs<'a>) -> HeaderResult<(Syntax<'a>, usize)> {
    let mut syntax: Option<Syntax<'_>> = None;
    let mut body_offset = 0;
    for pair in pairs {
        let rule = pair.as_rule();
        match rule {
            Rule::EOI => break,
            Rule::syntax => {
                syntax = Some(parse_syntax(pair)?);
            }
            Rule::body => {
                body_offset = pair.as_span().start();
            }

            _ => return Err(Error::UndefinedParsingRoute),
        }
    }
    syntax
        .ok_or(Error::UndefinedParsingRoute)
        .map(|syntax| (syntax, body_offset))
}

fn parse_syntax<'a>(pair: HeaderPair<'a>) -> HeaderResult<Syntax<'a>> {
    let pairs = pair.into_inner();
    let mut builder = Syntax::builder();
    for pair in pairs {
        let rule = pair.as_rule();
        match rule {
            Rule::COMMENT => {
                builder.with_comment(parse_comment(pair)?);
            }
            Rule::STRING_LIT => {
                builder.set_value(parse_literal_string(pair)?);
            }

            _ => return Err(Error::UndefinedParsingRoute),
        }
    }

    Ok(builder.build())
}
fn parse_comment<'a>(pair: HeaderPair<'a>) -> HeaderResult<Comment<'a>> {
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
fn parse_inner<'a, F, R>(pair: HeaderPair<'a>, rule: HeaderRule, handle: F) -> HeaderResult<R>
where
    F: FnOnce(HeaderPair<'a>) -> HeaderResult<R>,
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

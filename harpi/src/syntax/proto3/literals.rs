use std::borrow::Cow;

use pest::iterators::Pair;

use crate::{Error, model::Ident};

type LiteralRule = super::Rule;
type LiteralPair<'a> = Pair<'a, super::Rule>;
type LiteralResult<T> = Result<T, Error>;

pub(super) fn parse_literal_signed_int<'a>(pair: LiteralPair<'a>) -> LiteralResult<i64> {
    let pairs = pair.into_inner();
    let mut negative = false;
    for pair in pairs {
        let rule = pair.as_rule();
        match rule {
            LiteralRule::POSITIVE => {
                negative = false;
            }
            LiteralRule::NEGATIVE => {
                negative = true;
            }
            LiteralRule::INT_LIT => {
                return parse_literal_int(pair).map(|value| value * if negative { -1 } else { 1 });
            }
            _ => return Err(Error::UndefinedParsingRoute),
        }
    }
    Err(Error::UndefinedParsingRoute)
}
pub(super) fn parse_literal_unsigned_int<'a>(pair: LiteralPair<'a>) -> LiteralResult<u64> {
    let pair = pair
        .into_inner()
        .next()
        .ok_or(Error::UndefinedParsingRoute)?;
    let rule = pair.as_rule();
    match rule {
        LiteralRule::DECIMAL_LIT => Ok(pair.as_str().parse::<u64>()?),
        LiteralRule::OCTAL_LIT => parse_oct_unsigned(pair),
        LiteralRule::HEX_LIT => parse_hex_unsigned(pair),
        _ => Err(Error::UndefinedParsingRoute),
    }
}
pub(super) fn parse_literal_int<'a>(pair: LiteralPair<'a>) -> LiteralResult<i64> {
    let pair = pair
        .into_inner()
        .next()
        .ok_or(Error::UndefinedParsingRoute)?;
    let rule = pair.as_rule();
    match rule {
        LiteralRule::DECIMAL_LIT => Ok(pair.as_str().parse::<i64>()?),
        LiteralRule::OCTAL_LIT => parse_oct(pair),
        LiteralRule::HEX_LIT => parse_hex(pair),
        _ => Err(Error::UndefinedParsingRoute),
    }
}
pub(super) fn parse_oct_unsigned<'a>(pair: LiteralPair<'a>) -> LiteralResult<u64> {
    let pairs = pair.into_inner();
    let mut value = 0u64;
    let mut negative = false;
    for pair in pairs {
        let rule = pair.as_rule();
        match rule {
            LiteralRule::NEGATIVE => {
                negative = true;
            }
            LiteralRule::OCTAL_DIGIT => {
                value = (value << 3)
                    + (ascii_hex_to_int(pair.as_str().chars().next().unwrap() as u8) as u64);
            }
            _ => return Err(Error::UndefinedParsingRoute),
        }
    }
    Ok(value)
}
pub(super) fn parse_hex_unsigned<'a>(pair: LiteralPair<'a>) -> LiteralResult<u64> {
    let pairs = pair.into_inner();
    let mut value = 0u64;
    for pair in pairs {
        let rule = pair.as_rule();
        match rule {
            LiteralRule::HEX_DIGIT
            | LiteralRule::ZERO
            | LiteralRule::ONE
            | LiteralRule::DECIMAL_DIGIT => {
                value = (value << 4)
                    + (ascii_hex_to_int(pair.as_str().chars().next().unwrap() as u8) as u64);
            }

            _ => return Err(Error::UndefinedParsingRoute),
        }
    }
    Ok(value)
}
pub(super) fn parse_oct<'a>(pair: LiteralPair<'a>) -> LiteralResult<i64> {
    let pairs = pair.into_inner();
    let mut value = 0u64;
    let mut negative = false;
    for pair in pairs {
        let rule = pair.as_rule();
        match rule {
            LiteralRule::NEGATIVE => {
                negative = true;
            }
            LiteralRule::OCTAL_DIGIT => {
                value = (value << 3)
                    + (ascii_hex_to_int(pair.as_str().chars().next().unwrap() as u8) as u64);
            }
            _ => return Err(Error::UndefinedParsingRoute),
        }
    }
    Ok((value as i64) * (-1 * negative as i64))
}
pub(super) fn parse_hex<'a>(pair: LiteralPair<'a>) -> LiteralResult<i64> {
    let pairs = pair.into_inner();
    let mut value = 0u64;
    let mut negative = false;
    for pair in pairs {
        let rule = pair.as_rule();
        match rule {
            LiteralRule::NEGATIVE => {
                negative = true;
            }
            LiteralRule::HEX_DIGIT
            | LiteralRule::ZERO
            | LiteralRule::ONE
            | LiteralRule::DECIMAL_DIGIT => {
                value = (value << 4)
                    + (ascii_hex_to_int(pair.as_str().chars().next().unwrap() as u8) as u64);
            }

            _ => return Err(Error::UndefinedParsingRoute),
        }
    }
    Ok((value as i64) * (-1 * negative as i64))
}
pub(super) fn parse_bool<'a>(pair: LiteralPair<'a>) -> LiteralResult<bool> {
    match pair.as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(Error::UndefinedParsingRoute),
    }
}
/// Parses a literal string from the proto text. The string will resolve to a slice of the test if
/// no special escape sequences exist in the string, otherwise it will allocate a new escaped
/// string. The allocation will be initialized with the capacity of the unescaped string, since the
/// escaped string should be less than or equal to the old string in length.
pub(super) fn parse_literal_string<'a>(pair: LiteralPair<'a>) -> LiteralResult<Cow<'a, str>> {
    fn parse_literal_string_content<'b>(pair: LiteralPair<'b>) -> LiteralResult<Cow<'b, str>> {
        let str = pair.as_str();
        let mut offset = 0;
        let mut use_slice = true;
        let mut output = String::with_capacity(&pair.as_span().end() - pair.as_span().start());
        let pairs = pair.into_inner();
        for pair in pairs {
            let rule = pair.as_rule();
            match rule {
                LiteralRule::STRING_LIT_INNER => {
                    let pairs = pair.into_inner();
                    for pair in pairs {
                        let rule = pair.as_rule();
                        match rule {
                            LiteralRule::UNICODE_LONG_ESCAPE => {
                                let value = parse_hex(pair)? as u32;
                                if use_slice {
                                    output.push_str(&str[0..offset]);
                                    use_slice = false;
                                }
                                output.push(unsafe { char::from_u32_unchecked(value) });
                            }
                            LiteralRule::UNICODE_ESCAPE => {
                                let value = parse_hex(pair)? as u32;
                                if use_slice {
                                    output.push_str(&str[0..offset]);
                                    use_slice = false;
                                }
                                output.push(unsafe { char::from_u32_unchecked(value) });
                            }
                            LiteralRule::CHAR_ESCAPE => {
                                let value = pair.as_str();
                                if use_slice {
                                    output.push_str(&str[0..offset]);
                                    use_slice = false;
                                }
                                match value {
                                    r#"\a"# => output.push(7 as char),
                                    r#"\b"# => output.push(8 as char),
                                    r#"\t"# => output.push(9 as char),
                                    r#"\n"# => output.push(10 as char),
                                    r#"\v"# => output.push(11 as char),
                                    r#"\f"# => output.push(12 as char),
                                    r#"\r"# => output.push(13 as char),
                                    r#"\\"# => output.push('\\'),
                                    r#"\'"# => output.push('\''),
                                    r#"\""# => output.push('"'),
                                    _ => {}
                                }
                            }
                            LiteralRule::OCT_ESCAPE => {
                                let value = parse_oct(pair)? as u32;
                                if use_slice {
                                    output.push_str(&str[0..offset]);
                                    use_slice = false;
                                }
                                output.push(unsafe { char::from_u32_unchecked(value) });
                            }
                            LiteralRule::HEX_ESCAPE => {
                                let value = parse_hex(pair)? as u32;
                                if use_slice {
                                    output.push_str(&str[0..offset]);
                                    use_slice = false;
                                }
                                output.push(unsafe { char::from_u32_unchecked(value) });
                            }
                            LiteralRule::CHAR_OTHER => {
                                if use_slice {
                                    offset += 1;
                                } else {
                                    output.push_str(pair.as_str());
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => return Err(Error::UndefinedParsingRoute),
            }
        }
        if use_slice {
            Ok(Cow::Borrowed(str))
        } else {
            Ok(Cow::Owned(output))
        }
    }
    let pairs = pair.into_inner();
    let mut output = None;
    for pair in pairs {
        let rule = pair.as_rule();
        match rule {
            LiteralRule::STRING_LIT_CONTENT => {
                if output.is_none() {
                    output = Some(parse_literal_string_content(pair)?);
                } else {
                    return Err(Error::UndefinedParsingRoute);
                }
            }
            _ => return Err(Error::UndefinedParsingRoute),
        }
    }
    output.ok_or(Error::UndefinedParsingRoute)
}

pub(super) fn parse_float<'a>(pair: LiteralPair<'a>) -> LiteralResult<f64> {
    let str = pair.as_str();
    if str == "inf" {
        Ok(f64::INFINITY)
    } else if str == "nan" {
        Ok(f64::NAN)
    } else {
        Ok(pair.as_str().parse::<f64>()?)
    }
}
pub(super) fn parse_signed_float<'a>(pair: LiteralPair<'a>) -> LiteralResult<f64> {
    let pairs = pair.into_inner();
    let mut negative = false;
    for pair in pairs {
        let rule = pair.as_rule();
        match rule {
            LiteralRule::POSITIVE => {
                negative = false;
            }
            LiteralRule::NEGATIVE => {
                negative = true;
            }
            LiteralRule::FLOAT_LIT => {
                return parse_float(pair).map(|value| value * if negative { -1.0 } else { 1.0 });
            }
            _ => return Err(Error::UndefinedParsingRoute),
        }
    }
    Err(Error::UndefinedParsingRoute)
}
pub(super) fn parse_ident<'a>(pair: LiteralPair<'a>) -> LiteralResult<Ident<'a>> {
    let pairs = pair.into_inner();
    let mut relative = false;
    for pair in pairs {
        let rule = pair.as_rule();

        match rule {
            LiteralRule::LEADING_DOT => {
                relative = true;
            }
            LiteralRule::IDENT | LiteralRule::FULL_IDENT => {
                return Ok(Ident::new(relative, pair.as_str()));
            }
            LiteralRule::BRACED_FULL_IDENT => {
                let mut result = parse_ident(pair)?;
                result.set_relative(relative);
                return Ok(result);
            }
            _ => return Err(Error::UndefinedParsingRoute),
        }
    }
    Err(Error::UndefinedParsingRoute)
}

fn ascii_hex_to_int(value: u8) -> u8 {
    (((value & 0b01110000 != 0) as u8) * (value & 0b00001111))
        + (((value & 0b01000000 != 0) as u8) * 0b1001)
}

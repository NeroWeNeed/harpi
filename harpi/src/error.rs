use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ParsingSyntax(#[from] pest::error::Error<crate::syntax::proto3::Rule>),
    #[error(transparent)]
    ParsingProto3(#[from] pest::error::Error<crate::syntax::header::Rule>),
    #[error("Unknown error")]
    Unknown,
    #[error("path should be unreachable")]
    UndefinedParsingRoute,
    #[error("parser {0} is undefined")]
    UndefinedParser(String),
    #[error("parser {0} was expected, found {1}")]
    InvalidSyntax(String, String),
    #[error(transparent)]
    ParsingLiteralInt(#[from] ParseIntError),
    #[error(transparent)]
    ParsingLiteralFloat(#[from] ParseFloatError),
    #[error(transparent)]
    ParsingOption(#[from] crate::model::OptionBuilderError),
    #[error(transparent)]
    ParsingNormalField(#[from] crate::model::NormalFieldBuilderError),
    #[error(transparent)]
    ParsingOneOfFieldItemError(#[from] crate::model::OneOfFieldItemBuilderError),
    #[error(transparent)]
    ParsingMapField(#[from] crate::model::MapFieldBuilderError),
    #[error(transparent)]
    ParsingMessage(#[from] crate::model::MessageBuilderError),
}

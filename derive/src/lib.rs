use quote::{ToTokens, quote};
use syn::{Attribute, DeriveInput, Token, Type, parse_macro_input, punctuated::Punctuated};

#[proc_macro_derive(ProtoParser, attributes(parser))]
pub fn derive_parser(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as DeriveInput);
    let ident = item.ident;
    let generics = item.generics;
    let (impls, tys, wheres) = generics.split_for_impl();
    match TryInto::<ProtoParserAttributes>::try_into(item.attrs) {
        Ok(attrs) => {
            let parser = attrs.parsers;
            quote! {
                impl #impls harpi::ProtoParser for #ident #tys #wheres {
                    const SYNTAX: &'static str = "";
                    fn parse<'a, Visitor>(
                        data: &'a str,
                        visitor: &mut Visitor,
                    ) -> Result<(), harpi::Error>
                    where
                        Visitor: harpi::ProtoVisitor {
                        let (syntax,body) = harpi::header::parse_header(data)?;
                        match syntax.value().as_ref() {
                            #(<#parser>::SYNTAX => <#parser>::parse_with_syntax(body,syntax,visitor),)*
                                _ => Err(harpi::Error::UndefinedParser(syntax.value().to_string()))
                        }
                    }

                    fn parse_with_syntax<'a, Visitor>(
                        data: &'a str,
                        syntax: harpi::model::Syntax<'a>,
                        visitor: &mut Visitor,
                    ) -> Result<(), harpi::Error>
                    where
                        Visitor: harpi::ProtoVisitor {
                        let (parsed_syntax,body) = harpi::header::parse_header(data)?;
                        if syntax.value() != parsed_syntax.value() {
                            return Err(harpi::Error::InvalidSyntax(syntax.value().to_string(),parsed_syntax.value().to_string()));
                        }
                        match syntax.value().as_ref() {
                            #(<#parser>::SYNTAX => <#parser>::parse_with_syntax(body,syntax,visitor),)*
                                _ => Err(harpi::Error::UndefinedParser(syntax.value().to_string()))
                        }
                    }

                }
            }
            .into_token_stream()
            .into()
        }
        Err(err) => err.to_compile_error().into(),
    }
}

struct ProtoParserAttributes {
    parsers: Vec<Type>,
}
impl TryFrom<Vec<Attribute>> for ProtoParserAttributes {
    type Error = syn::Error;

    fn try_from(value: Vec<Attribute>) -> Result<Self, Self::Error> {
        let mut result = Vec::new();
        for attr in value {
            if attr.path().is_ident("parser") {
                let args =
                    attr.parse_args_with(Punctuated::<syn::Type, Token![,]>::parse_terminated)?;
                result.extend(args.into_iter());
            }
        }
        Ok(ProtoParserAttributes { parsers: result })
    }
}

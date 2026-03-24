//! NAPI bindings for [`nom_nrql`].

#![deny(clippy::all)]

mod convert;
mod napi_ast;

pub use napi_ast::*;

use napi::bindgen_prelude::*;
use napi_derive::napi;
use nom_nrql::ParseError;

/// Parse an NRQL string into a typed AST. Throws on syntax errors.
#[napi(js_name = "parseNrql")]
pub fn parse_nrql_js(input: String) -> Result<Query> {
    nom_nrql::parse_nrql(&input)
        .map(Into::into)
        .map_err(parse_error_to_napi)
}

fn parse_error_to_napi(e: ParseError) -> Error {
    let msg = match e.offset {
        Some(off) => format!("parse error at offset {}: {}", off, e.message),
        None => format!("parse error: {}", e.message),
    };
    Error::from_reason(msg)
}

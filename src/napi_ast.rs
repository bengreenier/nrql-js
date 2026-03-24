//! NAPI-facing AST types (parallel to `nom_nrql::ast`); shapes drive generated `index.d.ts`.
//!
//! Uses `i64` where `nom_nrql` uses `u64` — N-API bindgen supports signed integers for JS numbers.
//! `SelectArg` is a struct + `kind` (not a Rust recursive enum) so we can use `Box<SelectArg>` for
//! named-argument nesting with explicit `ToNapiValue` / `FromNapiValue` for `Box<SelectArg>`.

use napi::bindgen_prelude::*;
use napi_derive::napi;

/// A complete NRQL query.
#[napi(object)]
pub struct Query {
    pub select: SelectClause,
    pub from: FromClause,
    #[napi(js_name = "where")]
    pub where_clause: Option<WhereClause>,
    pub facet: Option<FacetClause>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub since: Option<TimeExpr>,
    pub until: Option<TimeExpr>,
    pub timeseries: Option<TimeseriesClause>,
    pub order_by: Option<OrderByClause>,
    pub with_timezone: Option<String>,
    pub compare_with: Option<TimeExpr>,
}

#[napi(object)]
pub struct SelectClause {
    pub items: Vec<SelectItem>,
}

#[napi]
pub enum SelectItem {
    Wildcard,
    Attr(AttributeRef),
    Function {
        name: String,
        args: Vec<SelectArg>,
        alias: Option<String>,
    },
}

#[napi(string_enum)]
pub enum SelectArgKind {
    Wildcard,
    Literal,
    Function,
    Named,
    WhereCondition,
    TimeInterval,
    Attribute,
}

/// One function/select argument (mirrors `nom_nrql::ast::SelectArg` with a flat shape for NAPI).
#[napi(object)]
pub struct SelectArg {
    pub kind: SelectArgKind,
    pub literal: Option<Literal>,
    pub function: Option<FunctionCall>,
    pub named: Option<NamedSelectArg>,
    pub where_condition: Option<Condition>,
    pub time_interval: Option<TimeInterval>,
    pub attribute: Option<AttributeRef>,
}

#[napi(object)]
pub struct NamedSelectArg {
    pub name: String,
    #[napi(ts_type = "SelectArg")]
    pub value: Box<SelectArg>,
}

#[napi(object)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<SelectArg>,
}

#[napi(object)]
pub struct AttributeRef {
    pub name: String,
}

#[napi(object)]
pub struct FromClause {
    pub event_types: Vec<String>,
}

#[napi(object)]
pub struct WhereClause {
    pub conditions: Vec<Condition>,
}

#[napi(object)]
pub struct Condition {
    pub attribute: AttributeRef,
    pub op: ComparisonOp,
    pub values: Option<Vec<Literal>>,
}

#[napi]
pub enum ComparisonOp {
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
    In,
    NotIn,
    Like,
    NotLike,
    IsNull,
    IsNotNull,
}

#[napi(object)]
pub struct FacetClause {
    pub attributes: Vec<FacetItem>,
    pub order_by: Option<OrderByClause>,
}

#[napi]
pub enum FacetItem {
    Attr(AttributeRef),
    Function(FunctionCall),
    Cases(Vec<FacetCase>),
}

#[napi(object)]
pub struct FacetCase {
    pub condition: Condition,
    pub alias: Option<String>,
}

#[napi(object)]
pub struct OrderByClause {
    pub items: Vec<OrderByItem>,
    pub limit: Option<i64>,
}

#[napi(object)]
pub struct OrderByItem {
    pub attribute_or_function: EitherAttrOrFunction,
    pub direction: Option<OrderDirection>,
}

#[napi]
pub enum EitherAttrOrFunction {
    Attr(AttributeRef),
    Function(FunctionCall),
}

#[napi]
pub enum OrderDirection {
    Asc,
    Desc,
}

#[napi]
pub enum TimeExpr {
    Relative { n: i64, unit: TimeUnit },
    Absolute { value: String },
    UnixMillis { value: i64 },
    Now,
}

#[napi(object)]
pub struct TimeInterval {
    pub n: i64,
    pub unit: TimeUnit,
}

#[napi]
pub enum TimeUnit {
    Millisecond,
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Quarter,
    Year,
}

#[napi]
pub enum Literal {
    String(String),
    Number(NumberLiteral),
    Bool(bool),
    Null,
}

#[napi]
pub enum NumberLiteral {
    Int(i64),
    Float(f64),
}

#[napi(object)]
pub struct TimeseriesClause {
    pub kind: TimeseriesKind,
    pub extrapolate: bool,
}

#[napi]
pub enum TimeseriesKind {
    Auto,
    Interval { n: i64, unit: TimeUnit },
}

impl ToNapiValue for Box<SelectArg> {
    unsafe fn to_napi_value(env: napi::sys::napi_env, val: Self) -> Result<napi::sys::napi_value> {
        unsafe { SelectArg::to_napi_value(env, *val) }
    }
}

impl FromNapiValue for Box<SelectArg> {
    unsafe fn from_napi_value(
        env: napi::sys::napi_env,
        napi_val: napi::sys::napi_value,
    ) -> Result<Self> {
        unsafe { Ok(Box::new(SelectArg::from_napi_value(env, napi_val)?)) }
    }
}

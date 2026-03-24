//! `nom_nrql::ast` → NAPI AST types.

use crate::napi_ast::*;
use nom_nrql::ast as N;

fn u64_to_i64(n: u64) -> i64 {
    i64::try_from(n).unwrap_or(i64::MAX)
}

impl From<N::Query> for Query {
    fn from(q: N::Query) -> Self {
        Self {
            select: q.select.into(),
            from: q.from.into(),
            where_clause: q.r#where.map(Into::into),
            facet: q.facet.map(Into::into),
            limit: q.limit.map(u64_to_i64),
            offset: q.offset.map(u64_to_i64),
            since: q.since.map(Into::into),
            until: q.until.map(Into::into),
            timeseries: q.timeseries.map(Into::into),
            order_by: q.order_by.map(Into::into),
            with_timezone: q.with_timezone,
            compare_with: q.compare_with.map(Into::into),
        }
    }
}

impl From<N::SelectClause> for SelectClause {
    fn from(s: N::SelectClause) -> Self {
        Self {
            items: s.items.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<N::SelectItem> for SelectItem {
    fn from(i: N::SelectItem) -> Self {
        match i {
            N::SelectItem::Wildcard => SelectItem::Wildcard,
            N::SelectItem::Attr(a) => SelectItem::Attr(a.into()),
            N::SelectItem::Function { name, args, alias } => SelectItem::Function {
                name,
                args: args.into_iter().map(Into::into).collect(),
                alias,
            },
        }
    }
}

impl From<N::SelectArg> for SelectArg {
    fn from(a: N::SelectArg) -> Self {
        match a {
            N::SelectArg::Wildcard => SelectArg {
                kind: SelectArgKind::Wildcard,
                literal: None,
                function: None,
                named: None,
                where_condition: None,
                time_interval: None,
                attribute: None,
            },
            N::SelectArg::Literal(l) => SelectArg {
                kind: SelectArgKind::Literal,
                literal: Some(l.into()),
                function: None,
                named: None,
                where_condition: None,
                time_interval: None,
                attribute: None,
            },
            N::SelectArg::Function(f) => SelectArg {
                kind: SelectArgKind::Function,
                literal: None,
                function: Some(f.into()),
                named: None,
                where_condition: None,
                time_interval: None,
                attribute: None,
            },
            N::SelectArg::Named { name, value } => SelectArg {
                kind: SelectArgKind::Named,
                literal: None,
                function: None,
                named: Some(NamedSelectArg {
                    name,
                    value: Box::new((*value).into()),
                }),
                where_condition: None,
                time_interval: None,
                attribute: None,
            },
            N::SelectArg::WhereCondition(c) => SelectArg {
                kind: SelectArgKind::WhereCondition,
                literal: None,
                function: None,
                named: None,
                where_condition: Some(c.into()),
                time_interval: None,
                attribute: None,
            },
            N::SelectArg::TimeInterval(t) => SelectArg {
                kind: SelectArgKind::TimeInterval,
                literal: None,
                function: None,
                named: None,
                where_condition: None,
                time_interval: Some(t.into()),
                attribute: None,
            },
            N::SelectArg::Attribute(a) => SelectArg {
                kind: SelectArgKind::Attribute,
                literal: None,
                function: None,
                named: None,
                where_condition: None,
                time_interval: None,
                attribute: Some(a.into()),
            },
        }
    }
}

impl From<N::FunctionCall> for FunctionCall {
    fn from(f: N::FunctionCall) -> Self {
        Self {
            name: f.name,
            args: f.args.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<N::AttributeRef> for AttributeRef {
    fn from(a: N::AttributeRef) -> Self {
        Self { name: a.name }
    }
}

impl From<N::FromClause> for FromClause {
    fn from(f: N::FromClause) -> Self {
        Self {
            event_types: f.event_types,
        }
    }
}

impl From<N::WhereClause> for WhereClause {
    fn from(w: N::WhereClause) -> Self {
        Self {
            conditions: w.conditions.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<N::Condition> for Condition {
    fn from(c: N::Condition) -> Self {
        Self {
            attribute: c.attribute.into(),
            op: c.op.into(),
            values: c.values.map(|vs| vs.into_iter().map(Into::into).collect()),
        }
    }
}

impl From<N::ComparisonOp> for ComparisonOp {
    fn from(o: N::ComparisonOp) -> Self {
        match o {
            N::ComparisonOp::Eq => ComparisonOp::Eq,
            N::ComparisonOp::Ne => ComparisonOp::Ne,
            N::ComparisonOp::Gt => ComparisonOp::Gt,
            N::ComparisonOp::Ge => ComparisonOp::Ge,
            N::ComparisonOp::Lt => ComparisonOp::Lt,
            N::ComparisonOp::Le => ComparisonOp::Le,
            N::ComparisonOp::In => ComparisonOp::In,
            N::ComparisonOp::NotIn => ComparisonOp::NotIn,
            N::ComparisonOp::Like => ComparisonOp::Like,
            N::ComparisonOp::NotLike => ComparisonOp::NotLike,
            N::ComparisonOp::IsNull => ComparisonOp::IsNull,
            N::ComparisonOp::IsNotNull => ComparisonOp::IsNotNull,
        }
    }
}

impl From<N::FacetClause> for FacetClause {
    fn from(f: N::FacetClause) -> Self {
        Self {
            attributes: f.attributes.into_iter().map(Into::into).collect(),
            order_by: f.order_by.map(Into::into),
        }
    }
}

impl From<N::FacetItem> for FacetItem {
    fn from(i: N::FacetItem) -> Self {
        match i {
            N::FacetItem::Attr(a) => FacetItem::Attr(a.into()),
            N::FacetItem::Function(f) => FacetItem::Function(f.into()),
            N::FacetItem::Cases(cases) => {
                FacetItem::Cases(cases.into_iter().map(Into::into).collect())
            }
        }
    }
}

impl From<N::FacetCase> for FacetCase {
    fn from(f: N::FacetCase) -> Self {
        Self {
            condition: f.condition.into(),
            alias: f.alias,
        }
    }
}

impl From<N::OrderByClause> for OrderByClause {
    fn from(o: N::OrderByClause) -> Self {
        Self {
            items: o.items.into_iter().map(Into::into).collect(),
            limit: o.limit.map(u64_to_i64),
        }
    }
}

impl From<N::OrderByItem> for OrderByItem {
    fn from(i: N::OrderByItem) -> Self {
        Self {
            attribute_or_function: i.attribute_or_function.into(),
            direction: i.direction.map(Into::into),
        }
    }
}

impl From<N::EitherAttrOrFunction> for EitherAttrOrFunction {
    fn from(e: N::EitherAttrOrFunction) -> Self {
        match e {
            N::EitherAttrOrFunction::Attr(a) => EitherAttrOrFunction::Attr(a.into()),
            N::EitherAttrOrFunction::Function(f) => EitherAttrOrFunction::Function(f.into()),
        }
    }
}

impl From<N::OrderDirection> for OrderDirection {
    fn from(d: N::OrderDirection) -> Self {
        match d {
            N::OrderDirection::Asc => OrderDirection::Asc,
            N::OrderDirection::Desc => OrderDirection::Desc,
        }
    }
}

impl From<N::TimeExpr> for TimeExpr {
    fn from(t: N::TimeExpr) -> Self {
        match t {
            N::TimeExpr::Relative { n, unit } => TimeExpr::Relative {
                n: u64_to_i64(n),
                unit: unit.into(),
            },
            N::TimeExpr::Absolute { value } => TimeExpr::Absolute { value },
            N::TimeExpr::UnixMillis { value } => TimeExpr::UnixMillis {
                value: u64_to_i64(value),
            },
            N::TimeExpr::Now => TimeExpr::Now,
        }
    }
}

impl From<N::TimeInterval> for TimeInterval {
    fn from(t: N::TimeInterval) -> Self {
        Self {
            n: u64_to_i64(t.n),
            unit: t.unit.into(),
        }
    }
}

impl From<N::TimeUnit> for TimeUnit {
    fn from(u: N::TimeUnit) -> Self {
        match u {
            N::TimeUnit::Millisecond => TimeUnit::Millisecond,
            N::TimeUnit::Second => TimeUnit::Second,
            N::TimeUnit::Minute => TimeUnit::Minute,
            N::TimeUnit::Hour => TimeUnit::Hour,
            N::TimeUnit::Day => TimeUnit::Day,
            N::TimeUnit::Week => TimeUnit::Week,
            N::TimeUnit::Month => TimeUnit::Month,
            N::TimeUnit::Quarter => TimeUnit::Quarter,
            N::TimeUnit::Year => TimeUnit::Year,
        }
    }
}

impl From<N::Literal> for Literal {
    fn from(l: N::Literal) -> Self {
        match l {
            N::Literal::String(s) => Literal::String(s),
            N::Literal::Number(n) => Literal::Number(n.into()),
            N::Literal::Bool(b) => Literal::Bool(b),
            N::Literal::Null => Literal::Null,
        }
    }
}

impl From<N::NumberLiteral> for NumberLiteral {
    fn from(n: N::NumberLiteral) -> Self {
        match n {
            N::NumberLiteral::Int(i) => NumberLiteral::Int(i),
            N::NumberLiteral::Float(f) => NumberLiteral::Float(f),
        }
    }
}

impl From<N::TimeseriesClause> for TimeseriesClause {
    fn from(t: N::TimeseriesClause) -> Self {
        Self {
            kind: t.kind.into(),
            extrapolate: t.extrapolate,
        }
    }
}

impl From<N::TimeseriesKind> for TimeseriesKind {
    fn from(k: N::TimeseriesKind) -> Self {
        match k {
            N::TimeseriesKind::Auto => TimeseriesKind::Auto,
            N::TimeseriesKind::Interval { n, unit } => TimeseriesKind::Interval {
                n: u64_to_i64(n),
                unit: unit.into(),
            },
        }
    }
}

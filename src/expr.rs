use std::borrow::Cow;

use chrono::{DateTime, Utc};
use rusqlite::{
    types::{ToSqlOutput, Value},
    vtab::array::Array,
};

use crate::{select::Select, Buildable};

pub struct CallExpr {
    pub func: &'static str,
    pub args: Vec<Expr>,
}

impl Expr_ for CallExpr {}

impl Buildable for CallExpr {
    fn build(&self, params: &mut Vec<ToSqlOutput<'_>>, out: &mut Vec<Cow<'static, str>>) {
        out.push(self.func.into());
        out.push("(".into());
        for (i, e) in self.args.iter().enumerate() {
            if i > 0 {
                out.push(",".into());
            }
            e.build(params, out);
        }
        out.push(")".into());
    }
}

pub struct ChainBinOpExpr {
    pub op: &'static str,
    pub clauses: Vec<Expr>,
}

impl Expr_ for ChainBinOpExpr {}

impl Buildable for ChainBinOpExpr {
    fn build(&self, params: &mut Vec<ToSqlOutput<'_>>, out: &mut Vec<Cow<'static, str>>) {
        out.push("(".into());
        for (i, e) in self.clauses.iter().enumerate() {
            if i > 0 {
                out.push(self.op.into());
            }
            e.build(params, out);
        }
        out.push(")".into());
    }
}

pub struct BinOpExpr {
    pub op: &'static str,
    pub l: Expr,
    pub r: Expr,
}

impl Expr_ for BinOpExpr {}

impl Buildable for BinOpExpr {
    fn build(&self, params: &mut Vec<ToSqlOutput<'_>>, out: &mut Vec<Cow<'static, str>>) {
        out.push("(".into());
        self.l.build(params, out);
        out.push(self.op.into());
        self.r.build(params, out);
        out.push(")".into());
    }
}

pub enum ParamExpr {
    V(Value),
    A(Array),
}

impl ParamExpr {
    pub fn from_date(d: DateTime<Utc>) -> ParamExpr {
        return ParamExpr::V(
            d.to_rfc3339_opts(chrono::SecondsFormat::Millis, false)
                .into(),
        );
    }
}

impl Expr_ for ParamExpr {}

impl Buildable for ParamExpr {
    fn build(&self, params: &mut Vec<ToSqlOutput<'_>>, out: &mut Vec<Cow<'static, str>>) {
        params.push(match self {
            ParamExpr::V(x) => ToSqlOutput::Owned(x.clone()),
            ParamExpr::A(x) => ToSqlOutput::Array(x.clone()),
        });
        out.push(format!("${}", params.len()).into());
    }
}

pub struct TupleExpr {
    pub values: Vec<Expr>,
}

impl Expr_ for TupleExpr {}

impl Buildable for TupleExpr {
    fn build(&self, params: &mut Vec<ToSqlOutput<'_>>, out: &mut Vec<Cow<'static, str>>) {
        out.push("(".into());
        for (i, v) in self.values.iter().enumerate() {
            if i > 0 {
                out.push(",".into());
            }
            v.build(params, out);
        }
        out.push(")".into());
    }
}

pub struct LiteralExpr(pub &'static str);

impl Expr_ for LiteralExpr {}

impl Buildable for LiteralExpr {
    fn build(&self, _params: &mut Vec<ToSqlOutput<'_>>, out: &mut Vec<Cow<'static, str>>) {
        out.push(self.0.into());
    }
}

pub struct NullExpr {}

impl Expr_ for NullExpr {}

impl Buildable for NullExpr {
    fn build(&self, _params: &mut Vec<ToSqlOutput<'_>>, out: &mut Vec<Cow<'static, str>>) {
        out.push("null".into());
    }
}

pub struct FieldExpr {
    pub table: &'static str,
    pub name: &'static str,
}

impl Expr_ for FieldExpr {}

impl Buildable for FieldExpr {
    fn build(&self, _params: &mut Vec<ToSqlOutput<'_>>, out: &mut Vec<Cow<'static, str>>) {
        out.push(format!("\"{}\".\"{}\"", self.table, self.name).into());
    }
}

pub struct SubselExpr {
    pub s: Select,
}

impl Expr_ for SubselExpr {}

impl Buildable for SubselExpr {
    fn build(&self, params: &mut Vec<ToSqlOutput<'_>>, out: &mut Vec<Cow<'static, str>>) {
        out.push("(".into());
        self.s.build(params, out);
        out.push(")".into());
    }
}

pub trait Expr_: Buildable {}

pub struct Expr(Box<dyn Expr_>);

impl Buildable for Expr {
    fn build(&self, params: &mut Vec<ToSqlOutput<'_>>, out: &mut Vec<Cow<'static, str>>) {
        self.0.build(params, out);
    }
}

impl Expr {
    pub fn new<T: Expr_ + 'static>(v: T) -> Expr {
        return Expr(Box::new(v));
    }
}

use std::borrow::Cow;

use rusqlite::types::ToSqlOutput;

use crate::{
    expr::{Expr, ParamExpr},
    select::Select,
    Buildable,
};

pub enum Table {
    Real(&'static str),
    // Virtual table with one column, "value"
    Virtual(ParamExpr),
}

impl Buildable for Table {
    fn build(&self, params: &mut Vec<ToSqlOutput<'_>>, out: &mut Vec<Cow<'static, str>>) {
        match &self {
            Table::Real(n) => {
                out.push((*n).into());
            }
            Table::Virtual(t) => {
                out.push("rarray(".into());
                t.build(params, out);
                out.push(")".into());
            }
        };
    }
}

pub enum JoinSource {
    Subsel(Select),
    Table(Table),
}

pub struct NamedJoinSource {
    pub source: JoinSource,
    pub alias: &'static str,
}

impl Buildable for NamedJoinSource {
    fn build(&self, params: &mut Vec<ToSqlOutput<'_>>, out: &mut Vec<Cow<'static, str>>) {
        match &self.source {
            JoinSource::Subsel(s) => {
                out.push("(".into());
                s.build(params, out);
                out.push(")".into());
            }
            JoinSource::Table(t) => {
                t.build(params, out);
            }
        };
        out.push("as".into());
        out.push(format!("\"{}\"", self.alias).into());
    }
}

pub struct Join {
    pub source: Box<NamedJoinSource>,
    pub on: Expr,
}

impl Buildable for Join {
    fn build(&self, params: &mut Vec<ToSqlOutput<'_>>, out: &mut Vec<Cow<'static, str>>) {
        out.push("left".into());
        out.push("join".into());
        self.source.as_ref().build(params, out);
        out.push("on".into());
        self.on.build(params, out);
    }
}

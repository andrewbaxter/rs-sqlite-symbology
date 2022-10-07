use std::borrow::Cow;

use rusqlite::types::ToSqlOutput;

use crate::{expr::Expr, join::NamedJoinSource, Buildable};

pub struct Update {
    pub table: &'static str,
    pub set: Vec<(&'static str, Expr)>,
    pub from: Vec<NamedJoinSource>,
    pub filter: Expr,
}

impl Update {
    pub fn build_str(self, params: &mut Vec<ToSqlOutput<'_>>) -> String {
        let mut out: Vec<Cow<'static, str>> = vec![];
        self.build(params, &mut out);
        return out.join(" ");
    }
}

impl Buildable for Update {
    fn build(&self, params: &mut Vec<ToSqlOutput<'_>>, out: &mut Vec<Cow<'static, str>>) {
        out.push("update".into());
        out.push(format!("\"{}\"", self.table).into());
        out.push("set".into());
        for (i, o) in self.set.iter().enumerate() {
            if i > 0 {
                out.push(",".into());
            }
            out.push(format!("\"{}\"", o.0).into());
            out.push("=".into());
            o.1.build(params, out);
        }
        if self.from.len() > 0 {
            out.push("from".into());
            for je in &self.from {
                je.build(params, out);
            }
        }
        out.push("where".into());
        self.filter.build(params, out);
    }
}

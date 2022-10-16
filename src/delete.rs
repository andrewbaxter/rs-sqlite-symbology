use std::borrow::Cow;

use rusqlite::types::ToSqlOutput;

use crate::{expr::Expr, Buildable};

pub struct Delete {
    pub table: &'static str,
    pub filter: Option<Expr>,
}

impl Delete {
    pub fn build_str(self, params: &mut Vec<ToSqlOutput<'_>>) -> String {
        let mut out: Vec<Cow<'static, str>> = vec![];
        self.build(params, &mut out);
        return out.join(" ");
    }
}

impl Buildable for Delete {
    fn build(&self, params: &mut Vec<ToSqlOutput<'_>>, out: &mut Vec<Cow<'static, str>>) {
        out.push("delete".into());
        out.push("from".into());
        out.push(format!("\"{}\"", self.table).into());
        out.push("where".into());
        if let Some(filter) = &self.filter {
            filter.build(params, out);
        }
    }
}

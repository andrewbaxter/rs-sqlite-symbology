use std::borrow::Cow;

use rusqlite::types::ToSqlOutput;

use crate::{expr::Expr, join::Join, Buildable};

#[derive(Copy, Clone)]
pub enum Order {
    Asc,
    Desc,
}
impl Buildable for Order {
    fn build(&self, _params: &mut Vec<ToSqlOutput<'_>>, out: &mut Vec<Cow<'static, str>>) {
        out.push(match self {
            Self::Asc => "asc".into(),
            Self::Desc => "desc".into(),
        });
    }
}

pub struct Select {
    pub table: &'static str,
    pub output: Vec<Expr>,
    pub join: Vec<Join>,
    pub filter: Option<Expr>,
    pub group: Vec<Expr>,
    pub order: Vec<(Expr, Order)>,
    pub limit: Option<usize>,
}

impl Select {
    pub fn build_str(self, params: &mut Vec<ToSqlOutput<'_>>) -> String {
        let mut out: Vec<Cow<'static, str>> = vec![];
        self.build(params, &mut out);
        return out.join(" ");
    }
}

impl Buildable for Select {
    fn build(&self, params: &mut Vec<ToSqlOutput<'_>>, out: &mut Vec<Cow<'static, str>>) {
        out.push("select".into());
        for (i, o) in self.output.iter().enumerate() {
            if i > 0 {
                out.push(",".into());
            }
            o.build(params, out);
        }
        out.push("from".into());
        out.push(format!("\"{}\"", self.table).into());
        for je in &self.join {
            je.build(params, out);
        }
        out.push("where".into());
        if let Some(filter) = &self.filter {
            filter.build(params, out);
        }
        if self.group.len() > 0 {
            out.push("group by".into());
            for (i, g) in self.group.iter().enumerate() {
                if i > 0 {
                    out.push(",".into());
                }
                g.build(params, out);
            }
        }
        if self.order.len() > 0 {
            out.push("order by".into());
            for (i, o) in self.order.iter().enumerate() {
                if i > 0 {
                    out.push(",".into());
                }
                o.0.build(params, out);
                o.1.build(params, out);
            }
        }
        if let Some(l) = self.limit {
            out.push("limit".into());
            out.push(format!("{}", l).into());
        }
    }
}

use std::borrow::Cow;

use rusqlite::types::ToSqlOutput;

use crate::{expr::Expr, Buildable};

pub struct Upsert {
    pub set: Vec<(&'static str, Expr)>,
}

pub enum Conflict {
    Update(Upsert),
    Ignore,
}

pub struct Insert {
    pub table: &'static str,
    pub values: Vec<(&'static str, Expr)>,
    pub conflict: Option<Conflict>,
    pub returning: Option<Expr>,
}

impl Insert {
    pub fn build_str(self, params: &mut Vec<ToSqlOutput<'_>>) -> String {
        let mut out: Vec<Cow<'static, str>> = vec![];
        self.build(params, &mut out);
        return out.join(" ");
    }
}

impl Buildable for Insert {
    fn build(&self, params: &mut Vec<ToSqlOutput<'_>>, out: &mut Vec<Cow<'static, str>>) {
        out.push("insert".into());
        out.push("into".into());
        out.push(format!("\"{}\"", self.table).into());
        out.push("(".into());
        for (i, o) in self.values.iter().enumerate() {
            if i > 0 {
                out.push(",".into());
            }
            out.push(format!("\"{}\"", o.0).into());
        }
        out.push(")".into());
        out.push("values".into());
        out.push("(".into());
        for (i, o) in self.values.iter().enumerate() {
            if i > 0 {
                out.push(",".into());
            }
            o.1.build(params, out);
        }
        out.push(")".into());
        if let Some(method) = &self.conflict {
            match method {
                Conflict::Update(upsert) => {
                    out.push("on".into());
                    out.push("conflict".into());
                    out.push("do".into());
                    out.push("update".into());
                    out.push("set".into());
                    for (i, o) in upsert.set.iter().enumerate() {
                        if i > 0 {
                            out.push(",".into());
                        }
                        out.push(format!("\"{}\"", o.0).into());
                        out.push("=".into());
                        o.1.build(params, out);
                    }
                }
                Conflict::Ignore => {
                    out.push("on".into());
                    out.push("conflict".into());
                    out.push("do".into());
                    out.push("nothing".into());
                }
            };
        }
        if let Some(returning) = &self.returning {
            out.push("returning".into());
            returning.build(params, out);
        }
    }
}

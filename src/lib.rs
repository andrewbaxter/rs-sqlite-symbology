use std::borrow::Cow;

use enum_dispatch::enum_dispatch;
use rusqlite::types::ToSqlOutput;

pub mod delete;
pub mod expr;
pub mod insert;
pub mod join;
pub mod select;
pub mod update;

#[enum_dispatch]
pub trait Buildable {
    fn build(&self, params: &mut Vec<ToSqlOutput<'_>>, out: &mut Vec<Cow<'static, str>>);
}

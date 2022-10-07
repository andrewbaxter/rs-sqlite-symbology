A Rust data model for Sqlite queries that has methods for building query strings and associated parameter lists.

Typically you'd make an instance of `Select` `Insert` or `Update`, then use it like

```
let mut params: Vec<ToSqlOutput<'_>> = vec![];
let query_str = Select{...}.build_str(params.as_mut());
let mut stmt = conn.prepare(&query_text).unwrap();
let mut rows = stmt.query(params_from_iter(params.iter())).unwrap();
...
```

Expressions are used in various places and are instantiated like

```
Expr::new(ParamExpr::V(
    bincode::serialize(&ch_id.owner).unwrap().into(),
))
```

Benefits over hand written SQL:

- You can conditinally add/remove/modify clauses, compose query parts using standard language tools (functions) - useful for dynamic queries
- You can use variables for fields/tables/etc. guaranteeing consistency and preventing typos at compile time
- Automatic formatting using rustfmt

Benefits over ORMs:

- Not a framework

Benefits over other SQL generators:

- No macros, so automatic formatting using rustfmt
- SQLite specific, which (theoretically) means better language coverage

Ideally this would have 1-1 relation with the graphs on the SQLite website, sans maybe the formatting-only paths (like including/omitting "as").

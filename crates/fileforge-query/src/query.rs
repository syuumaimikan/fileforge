use fileforge_core::Result;
use fileforge_storage::{Reader, Record};

#[derive(Debug, Clone)]
pub enum Expr {
    Contains(String),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
}

impl Expr {
    pub fn eval(&self, text: &str) -> bool {
        match self {
            Expr::Contains(keyword) => text.contains(keyword),
            Expr::And(a, b) => a.eval(text) && b.eval(text),
            Expr::Or(a, b) => a.eval(text) || b.eval(text),
            Expr::Not(expr) => !expr.eval(text),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Query {
    pub expr: Expr,
    pub limit: usize,
}

impl Query {
    pub fn new(keyword: impl Into<String>, limit: usize) -> Self {
        Self { expr: Expr::Contains(keyword.into()), limit }
    }

    pub fn from_expr(expr: Expr, limit: usize) -> Self { Self { expr, limit } }
}

pub struct QueryEngine;

impl QueryEngine {
    pub fn run<R, F>(reader: &mut R, query: Query, mut on_record: F) -> Result<usize>
    where
        R: Reader + ?Sized,
        F: for<'a> FnMut(Record<'a>),
    {
        let mut count = 0usize;
        while let Some(record) = reader.next_record()? {
            if query.expr.eval(record.data.as_ref()) {
                on_record(record);
                count += 1;
                if query.limit > 0 && count >= query.limit { break; }
            }
        }
        Ok(count)
    }
}

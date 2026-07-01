use fileforge_core::Result;

use crate::{Expr, Query};

pub struct QueryParser;

impl QueryParser {
    pub fn parse(input: &str, limit: usize) -> Result<Query> {
        let input = input.trim();
        if input.starts_with("contains(") && input.ends_with(')') {
            let keyword = parse_contains(input)?;
            return Ok(Query::new(keyword, limit));
        }
        let expr = parse_simple_expr(input)?;
        Ok(Query::from_expr(expr, limit))
    }
}

fn parse_simple_expr(input: &str) -> Result<Expr> {
    let tokens = input.split_whitespace().collect::<Vec<_>>();
    if tokens.is_empty() { anyhow::bail!("empty query"); }
    parse_or(&tokens)
}

fn parse_or(tokens: &[&str]) -> Result<Expr> {
    if let Some(pos) = tokens.iter().position(|t| t.eq_ignore_ascii_case("OR")) {
        let left = parse_or(&tokens[..pos])?;
        let right = parse_and(&tokens[pos + 1..])?;
        return Ok(Expr::Or(Box::new(left), Box::new(right)));
    }
    parse_and(tokens)
}

fn parse_and(tokens: &[&str]) -> Result<Expr> {
    if let Some(pos) = tokens.iter().position(|t| t.eq_ignore_ascii_case("AND")) {
        let left = parse_and(&tokens[..pos])?;
        let right = parse_not(&tokens[pos + 1..])?;
        return Ok(Expr::And(Box::new(left), Box::new(right)));
    }
    parse_not(tokens)
}

fn parse_not(tokens: &[&str]) -> Result<Expr> {
    if tokens.len() >= 2 && tokens[0].eq_ignore_ascii_case("NOT") {
        let expr = parse_atom(&tokens[1..])?;
        return Ok(Expr::Not(Box::new(expr)));
    }
    parse_atom(tokens)
}

fn parse_atom(tokens: &[&str]) -> Result<Expr> {
    if tokens.len() != 1 { anyhow::bail!("invalid expression"); }
    let token = tokens[0];
    if token.starts_with("contains(") { Ok(Expr::Contains(parse_contains(token)?)) }
    else { Ok(Expr::Contains(token.to_string())) }
}

fn parse_contains(input: &str) -> Result<String> {
    if !(input.starts_with("contains(") && input.ends_with(')')) { anyhow::bail!("invalid contains()"); }
    let inner = &input["contains(".len()..input.len() - 1];
    let parts = split_args(inner);
    if parts.len() != 2 { anyhow::bail!("contains() requires 2 arguments"); }
    if parts[0].trim() != "text" { anyhow::bail!("only contains(text, \"...\") is supported now"); }
    unquote(parts[1].trim())
}

fn split_args(input: &str) -> Vec<&str> {
    let mut args = Vec::new();
    let mut start = 0usize;
    let mut in_string = false;
    for (i, c) in input.char_indices() {
        match c {
            '"' => in_string = !in_string,
            ',' if !in_string => { args.push(input[start..i].trim()); start = i + 1; }
            _ => {}
        }
    }
    args.push(input[start..].trim());
    args
}

fn unquote(input: &str) -> Result<String> {
    if input.starts_with('"') && input.ends_with('"') && input.len() >= 2 {
        Ok(input[1..input.len() - 1].to_string())
    } else {
        anyhow::bail!("string argument must be quoted")
    }
}

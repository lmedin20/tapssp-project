use anyhow::{Result, bail, Context};
#[derive(Clone, Copy, Debug)]
pub enum Op { Eq, Ne, Lt, Le, Gt, Ge }
#[derive(Debug)]
pub struct Pred {
    col: usize,
    op: Op,
    val: String,
}
#[derive(Debug)]
pub enum Expr {
    Single(Pred),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
}
pub fn parse(expr: &str, headers: &[String]) -> Result<Expr> {
    parse_or(expr.trim(), headers)
}
fn parse_or(expr: &str, headers: &[String]) -> Result<Expr> {
    if let Some(idx) = split_top(expr, " OR ") {
        let left = parse_or(&expr[..idx], headers)?;
        let right = parse_and(&expr[idx + 4..], headers)?;
        return Ok(Expr::Or(Box::new(left), Box::new(right)));
    }
    parse_and(expr, headers)
}
fn parse_and(expr: &str, headers: &[String]) -> Result<Expr> {
    if let Some(idx) = split_top(expr, " AND ") {
        let left = parse_and(&expr[..idx], headers)?;
        let right = parse_single(&expr[idx + 5..], headers)?;
        return Ok(Expr::And(Box::new(left), Box::new(right)));
    }
    parse_single(expr, headers)
}
fn parse_single(expr: &str, headers: &[String]) -> Result<Expr> {
    let expr = expr.trim();
    if expr.starts_with('(') && expr.ends_with(')') {
        return parse(&expr[1..expr.len() - 1], headers);
    }
    let ops = [(">=", Op::Ge),("<=", Op::Le),("==", Op::Eq),("!=", Op::Ne),(">",  Op::Gt),("<",  Op::Lt),];
    let (op_str, op) = ops
        .iter()
        .find_map(|(s, o)| expr.find(s).map(|_| (*s, *o)))
        .context("expected comparison operator")?;
    let parts: Vec<&str> = expr.splitn(2, op_str).collect();
    let col = parts[0].trim();
    let val = parts[1].trim().trim_matches('"').trim_matches('\'').to_string();
    let col_idx = headers
        .iter()
        .position(|h| h == col)
        .with_context(|| format!("unknown column: {col}"))?;
    Ok(Expr::Single(Pred { col: col_idx, op, val }))
}
impl Pred {
    pub fn eval(&self, rec: &[String]) -> Result<bool> {
        let cell = rec.get(self.col).map(|s| s.as_str()).unwrap_or("");
        let a_num = cell.parse::<f64>();
        let b_num = self.val.parse::<f64>();
        if a_num.is_ok() && b_num.is_ok() {
            let (a, b) = (a_num.unwrap(), b_num.unwrap());
            return Ok(match self.op {
                Op::Eq => a == b,
                Op::Ne => a != b,
                Op::Lt => a < b,
                Op::Le => a <= b,
                Op::Gt => a > b,
                Op::Ge => a >= b,
            });
        }
        match self.op {
            Op::Lt | Op::Le | Op::Gt | Op::Ge => Ok(false),
            Op::Eq => Ok(cell == self.val),
            Op::Ne => Ok(cell != self.val),
        }
    }
}
impl Expr {
    pub fn eval(&self, rec: &[String]) -> Result<bool> {
        match self {
            Expr::Single(p) => p.eval(rec),
            Expr::And(a, b) => Ok(a.eval(rec)? && b.eval(rec)?),
            Expr::Or(a, b) => Ok(a.eval(rec)? || b.eval(rec)?),
        }
    }
}
fn split_top(expr: &str, op: &str) -> Option<usize> {
    let chars: Vec<char> = expr.chars().collect();
    let op_chars: Vec<char> = op.chars().collect();
    let mut depth = 0;
    for i in 0..chars.len() {
        match chars[i] {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {},
        }
        if depth == 0 && i + op_chars.len() <= chars.len() {
            if chars[i..i + op_chars.len()] == op_chars[..] {
                return Some(i);
            }
        }
    }
    None
}





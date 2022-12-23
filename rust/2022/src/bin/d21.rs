use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum DeferredExpr {
    Num(i64),
    // assumes there are never mixed expressions such as
    // a + 1 or 2 + 2 or else we'd need to box each branch
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

impl FromStr for DeferredExpr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_whitespace().collect();

        match parts.len() {
            1 => {
                let e = parts[0].trim();
                if let Ok(i) = e.parse::<i64>() {
                    Ok(DeferredExpr::Num(i))
                } else {
                    Err(format!("Invalid value, expected integer found: {}", e))
                }
            }
            3 => {
                let (lhs, op, rhs) = (parts[0], parts[1], parts[2]);

                match op.trim() {
                    "+" => Ok(DeferredExpr::Add(
                        lhs.trim().to_string(),
                        rhs.trim().to_string(),
                    )),
                    "-" => Ok(DeferredExpr::Sub(
                        lhs.trim().to_string(),
                        rhs.trim().to_string(),
                    )),
                    "*" => Ok(DeferredExpr::Mul(
                        lhs.trim().to_string(),
                        rhs.trim().to_string(),
                    )),
                    "/" => Ok(DeferredExpr::Div(
                        lhs.trim().to_string(),
                        rhs.trim().to_string(),
                    )),
                    _ => Err(format!(
                        "Invalid expression operator `{}` in expression: {}",
                        op, s
                    )),
                }
            }
            _ => Err(format!("Invalid expression: {}", s)),
        }
    }
}

fn parse_line(s: &str) -> Option<(String, DeferredExpr)> {
    let (name, expr) = s.split_once(":")?;

    if let Ok(e) = expr.parse::<DeferredExpr>() {
        Some((name.to_string(), e))
    } else {
        None
    }
}

fn eval(vars: &HashMap<String, DeferredExpr>, expr: &DeferredExpr) -> Result<i64, String> {
    use DeferredExpr::*;

    match expr {
        Num(i) => Ok(i.to_owned()),
        Add(lhs, rhs) => {
            let lhs = vars
                .get(lhs)
                .ok_or(format!("Undeclared variable: {}", lhs))?;
            let rhs = vars
                .get(rhs)
                .ok_or(format!("Undeclared variable: {}", rhs))?;

            Ok(eval(vars, lhs)? + eval(vars, rhs)?)
        }
        Sub(lhs, rhs) => {
            let lhs = vars
                .get(lhs)
                .ok_or(format!("Undeclared variable: {}", lhs))?;
            let rhs = vars
                .get(rhs)
                .ok_or(format!("Undeclared variable: {}", rhs))?;

            Ok(eval(vars, lhs)? - eval(vars, rhs)?)
        }
        Mul(lhs, rhs) => {
            let lhs = vars
                .get(lhs)
                .ok_or(format!("Undeclared variable: {}", lhs))?;
            let rhs = vars
                .get(rhs)
                .ok_or(format!("Undeclared variable: {}", rhs))?;

            Ok(eval(vars, lhs)? * eval(vars, rhs)?)
        }
        Div(lhs, rhs) => {
            let lhs = vars
                .get(lhs)
                .ok_or(format!("Undeclared variable: {}", lhs))?;
            let rhs = vars
                .get(rhs)
                .ok_or(format!("Undeclared variable: {}", rhs))?;

            Ok(eval(vars, lhs)? / eval(vars, rhs)?)
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let input = read_to_string(path).unwrap();

    let vars = input
        .trim()
        .split("\n")
        .filter_map(parse_line)
        .collect::<HashMap<String, DeferredExpr>>();

    let root = vars.get("root").unwrap();
    let result = eval(&vars, root).unwrap();
    dbg!(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let (name, expr) = parse_line("x: 5").unwrap();

        assert_eq!(name, "x");
        assert_eq!(expr, DeferredExpr::Num(5));
    }

    #[test]
    fn test_def_expr_add() {
        let expr = "a + b".parse::<DeferredExpr>().unwrap();
        assert_eq!(expr, DeferredExpr::Add("a".to_string(), "b".to_string()));
    }

    #[test]
    fn test_def_expr_sub() {
        let expr = "a - b".parse::<DeferredExpr>().unwrap();
        assert_eq!(expr, DeferredExpr::Sub("a".to_string(), "b".to_string()));
    }

    #[test]
    fn test_def_expr_mul() {
        let expr = "a * b".parse::<DeferredExpr>().unwrap();
        assert_eq!(expr, DeferredExpr::Mul("a".to_string(), "b".to_string()));
    }

    #[test]
    fn test_def_expr_div() {
        let expr = "a / b".parse::<DeferredExpr>().unwrap();
        assert_eq!(expr, DeferredExpr::Div("a".to_string(), "b".to_string()));
    }

    #[test]
    fn test_eval() {
        let vars = [
            (
                "x".to_string(),
                DeferredExpr::Add("a".to_string(), "b".to_string()),
            ),
            ("d".to_string(), DeferredExpr::Num(10)),
            ("c".to_string(), DeferredExpr::Num(1)),
            (
                "a".to_string(),
                DeferredExpr::Mul("c".to_string(), "d".to_string()),
            ),
            ("b".to_string(), DeferredExpr::Num(5)),
        ]
        .into_iter()
        .collect::<HashMap<String, DeferredExpr>>();

        let root = vars.get("x").unwrap();
        let result = eval(&vars, &root).unwrap();

        assert_eq!(result, 15);
    }
}

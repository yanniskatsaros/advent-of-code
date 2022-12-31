use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
struct Ref(String);

#[derive(Debug, PartialEq, Clone)]
enum Expr {
    Num(String, i64),
    // assumes there are never mixed expressions such as
    // a + 1 or 2 + 2 or else we'd need to box each branch
    Add(String, Ref, Ref),
    Sub(String, Ref, Ref),
    Mul(String, Ref, Ref),
    Div(String, Ref, Ref),
}

impl Expr {
    /// Returns `true` if the expression tree has a branch with a variable name matching the given `target` name
    fn contains(&self, vars: &HashMap<String, Self>, target: &str) -> Result<bool, String> {
        match self {
            Self::Num(name, _) => Ok(name == target),
            Self::Add(_, Ref(lhs), Ref(rhs))
            | Self::Sub(_, Ref(lhs), Ref(rhs))
            | Self::Mul(_, Ref(lhs), Ref(rhs))
            | Self::Div(_, Ref(lhs), Ref(rhs)) => {
                let lhs = vars
                    .get(lhs)
                    .ok_or(format!("Undeclared variable: {}", lhs))?;
                let rhs = vars
                    .get(rhs)
                    .ok_or(format!("Undeclared variable: {}", rhs))?;

                Ok(lhs.contains(vars, target)? || rhs.contains(vars, target)?)
            }
        }
    }
}

impl FromStr for Expr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, expr) = s
            .split_once(":")
            .ok_or(format!("Invalid expression: {s}"))?;
        let name = name.to_string();

        let parts: Vec<_> = expr.split_whitespace().collect();
        match parts.len() {
            1 => {
                let e = parts[0].trim();
                if let Ok(i) = e.parse::<i64>() {
                    Ok(Expr::Num(name, i))
                } else {
                    Err(format!("Invalid value, expected integer found: {}", e))
                }
            }
            3 => {
                let (lhs, op, rhs) = (parts[0], parts[1], parts[2]);

                match op.trim() {
                    "+" => Ok(Expr::Add(
                        name,
                        Ref(lhs.trim().to_string()),
                        Ref(rhs.trim().to_string()),
                    )),
                    "-" => Ok(Expr::Sub(
                        name,
                        Ref(lhs.trim().to_string()),
                        Ref(rhs.trim().to_string()),
                    )),
                    "*" => Ok(Expr::Mul(
                        name,
                        Ref(lhs.trim().to_string()),
                        Ref(rhs.trim().to_string()),
                    )),
                    "/" => Ok(Expr::Div(
                        name,
                        Ref(lhs.trim().to_string()),
                        Ref(rhs.trim().to_string()),
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

fn eval(vars: &HashMap<String, Expr>, expr: &Expr) -> Result<i64, String> {
    use Expr::*;

    match expr {
        Num(_, i) => Ok(i.to_owned()),
        Add(_, Ref(lhs), Ref(rhs)) => {
            let lhs = vars
                .get(lhs)
                .ok_or(format!("Undeclared variable: {}", lhs))?;
            let rhs = vars
                .get(rhs)
                .ok_or(format!("Undeclared variable: {}", rhs))?;

            Ok(eval(vars, lhs)? + eval(vars, rhs)?)
        }
        Sub(_, Ref(lhs), Ref(rhs)) => {
            let lhs = vars
                .get(lhs)
                .ok_or(format!("Undeclared variable: {}", lhs))?;
            let rhs = vars
                .get(rhs)
                .ok_or(format!("Undeclared variable: {}", rhs))?;

            Ok(eval(vars, lhs)? - eval(vars, rhs)?)
        }
        Mul(_, Ref(lhs), Ref(rhs)) => {
            let lhs = vars
                .get(lhs)
                .ok_or(format!("Undeclared variable: {}", lhs))?;
            let rhs = vars
                .get(rhs)
                .ok_or(format!("Undeclared variable: {}", rhs))?;

            Ok(eval(vars, lhs)? * eval(vars, rhs)?)
        }
        Div(_, Ref(lhs), Ref(rhs)) => {
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

fn solve(
    vars: &HashMap<String, Expr>,
    expr: &Expr,
    unknown: &str,
    target: i64,
) -> Result<i64, String> {
    use Expr::*;

    match expr {
        Num(name, _) => {
            if name == unknown {
                Ok(target)
            } else {
                Err("not sure how we got here".to_string())
            }
        }
        Add(_, Ref(lhs), Ref(rhs)) => {
            let lhs = vars
                .get(lhs)
                .ok_or(format!("Undeclared variable: {}", lhs))?;
            let rhs = vars
                .get(rhs)
                .ok_or(format!("Undeclared variable: {}", rhs))?;

            if lhs.contains(vars, unknown)? {
                let rhs = eval(vars, rhs)?;
                let target = target - rhs;
                solve(vars, lhs, unknown, target)
            } else if rhs.contains(vars, unknown)? {
                let lhs = eval(vars, lhs)?;
                let target = target - lhs;
                solve(vars, rhs, unknown, target)
            } else {
                Err(format!("Unknown variable: {unknown}"))
            }
        }
        Sub(_, Ref(lhs), Ref(rhs)) => {
            let lhs = vars
                .get(lhs)
                .ok_or(format!("Undeclared variable: {}", lhs))?;
            let rhs = vars
                .get(rhs)
                .ok_or(format!("Undeclared variable: {}", rhs))?;

            if lhs.contains(vars, unknown)? {
                let rhs = eval(vars, rhs)?;
                let target = target + rhs;
                solve(vars, lhs, unknown, target)
            } else if rhs.contains(vars, unknown)? {
                let lhs = eval(vars, lhs)?;
                let target = lhs - target;
                solve(vars, rhs, unknown, target)
            } else {
                Err(format!("Unknown variable: {unknown}"))
            }
        }
        Mul(_, Ref(lhs), Ref(rhs)) => {
            let lhs = vars
                .get(lhs)
                .ok_or(format!("Undeclared variable: {}", lhs))?;
            let rhs = vars
                .get(rhs)
                .ok_or(format!("Undeclared variable: {}", rhs))?;

            if lhs.contains(vars, unknown)? {
                let rhs = eval(vars, rhs)?;
                let target = target / rhs;
                solve(vars, lhs, unknown, target)
            } else if rhs.contains(vars, unknown)? {
                let lhs = eval(vars, lhs)?;
                let target = target / lhs;
                solve(vars, rhs, unknown, target)
            } else {
                Err(format!("Unknown variable: {unknown}"))
            }
        }
        Div(_, Ref(lhs), Ref(rhs)) => {
            let lhs = vars
                .get(lhs)
                .ok_or(format!("Undeclared variable: {}", lhs))?;
            let rhs = vars
                .get(rhs)
                .ok_or(format!("Undeclared variable: {}", rhs))?;

            if lhs.contains(vars, unknown)? {
                let rhs = eval(vars, rhs)?;
                let target = target * rhs;
                solve(vars, lhs, unknown, target)
            } else if rhs.contains(vars, unknown)? {
                let lhs = eval(vars, lhs)?;
                let target = lhs / target;
                solve(vars, rhs, unknown, target)
            } else {
                Err(format!("Unknown variable: {unknown}"))
            }
        }
    }
}

fn part1(vars: &HashMap<String, Expr>) {
    let root = vars.get("root").unwrap();
    let result = eval(&vars, root).unwrap();
    println!("Part I: {result}");
}

fn part2(vars: &HashMap<String, Expr>) {
    let unknown = "humn";
    let root = vars.get("root").unwrap();
    let result = match root {
        Expr::Add(_, Ref(lhs), Ref(rhs)) => {
            let lhs = vars
                .get(lhs)
                .ok_or(format!("Undeclared variable: {}", lhs))
                .unwrap();
            let rhs = vars
                .get(rhs)
                .ok_or(format!("Undeclared variable: {}", rhs))
                .unwrap();

            if lhs.contains(vars, unknown).unwrap() {
                let target = eval(vars, rhs).unwrap();
                solve(vars, lhs, unknown, target).unwrap()
            } else if rhs.contains(vars, unknown).unwrap() {
                let target = eval(vars, lhs).unwrap();
                solve(vars, rhs, unknown, target).unwrap()
            } else {
                panic!("No binding found: humn");
            }
        }
        _ => unreachable!(),
    };
    println!("Part II: {result}");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let input = read_to_string(path).unwrap();
    let vars = input
        .trim()
        .split("\n")
        .filter_map(|s| {
            let expr = Expr::from_str(s).ok()?;
            match &expr {
                Expr::Num(name, _) => Some((name.clone(), expr.clone())),
                Expr::Add(name, _, _)
                | Expr::Sub(name, _, _)
                | Expr::Mul(name, _, _)
                | Expr::Div(name, _, _) => Some((name.clone(), expr.clone())),
            }
        })
        .collect::<HashMap<String, Expr>>();

    part1(&vars);
    part2(&vars);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_expr_num() {
        let expr = Expr::from_str("x: 5").unwrap();

        let (name, i) = match &expr {
            Expr::Num(name, i) => (name, *i),
            _ => panic!(),
        };

        assert_eq!(name, "x");
        assert_eq!(i, 5 as i64);
    }

    #[test]
    fn test_parse_expr_add() {
        let expr = Expr::from_str("x: a + b").unwrap();

        let (name, lhs, rhs) = match &expr {
            Expr::Add(name, l, r) => (name, l.clone(), r.clone()),
            _ => panic!(),
        };

        assert_eq!(name, "x");
        assert_eq!(lhs, Ref("a".to_string()));
        assert_eq!(rhs, Ref("b".to_string()));
    }

    #[test]
    fn test_parse_expr_sub() {
        let expr = Expr::from_str("x: a - b").unwrap();

        let (name, lhs, rhs) = match &expr {
            Expr::Sub(name, l, r) => (name, l.clone(), r.clone()),
            _ => panic!(),
        };

        assert_eq!(name, "x");
        assert_eq!(lhs, Ref("a".to_string()));
        assert_eq!(rhs, Ref("b".to_string()));
    }

    #[test]
    fn test_parse_expr_mul() {
        let expr = Expr::from_str("x: a * b").unwrap();

        let (name, lhs, rhs) = match &expr {
            Expr::Mul(name, l, r) => (name, l.clone(), r.clone()),
            _ => panic!(),
        };

        assert_eq!(name, "x");
        assert_eq!(lhs, Ref("a".to_string()));
        assert_eq!(rhs, Ref("b".to_string()));
    }

    #[test]
    fn test_parse_expr_div() {
        let expr = Expr::from_str("x: a / b").unwrap();

        let (name, lhs, rhs) = match &expr {
            Expr::Div(name, l, r) => (name, l.clone(), r.clone()),
            _ => panic!(),
        };

        assert_eq!(name, "x");
        assert_eq!(lhs, Ref("a".to_string()));
        assert_eq!(rhs, Ref("b".to_string()));
    }

    #[test]
    fn test_eval() {
        let vars = [
            (
                "x".to_string(),
                Expr::Add("x".to_string(), Ref("a".to_string()), Ref("b".to_string())),
            ),
            ("d".to_string(), Expr::Num("d".to_string(), 10)),
            ("c".to_string(), Expr::Num("c".to_string(), 1)),
            (
                "a".to_string(),
                Expr::Mul("a".to_string(), Ref("c".to_string()), Ref("d".to_string())),
            ),
            ("b".to_string(), Expr::Num("b".to_string(), 5)),
        ]
        .into_iter()
        .collect::<HashMap<String, Expr>>();

        let root = vars.get("x").unwrap();
        let result = eval(&vars, &root).unwrap();

        assert_eq!(result, 15);
    }
}

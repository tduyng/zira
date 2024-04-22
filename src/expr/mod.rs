pub mod binding_usage;

use crate::{
    env::Env,
    utils::{extract_digits, extract_whitespace, remove_tag},
    val::Val,
};

use self::binding_usage::BindingUsage;

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

impl Number {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, number) = extract_digits(s)?;
        Ok((s, Self(number.parse().unwrap())))
    }
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        remove_tag("+", s)
            .map(|s| (s, Self::Add))
            .or_else(|_| remove_tag("-", s).map(|s| (s, Self::Sub)))
            .or_else(|_| remove_tag("*", s).map(|s| (s, Self::Mul)))
            .or_else(|_| remove_tag("/", s).map(|s| (s, Self::Div)))
    }
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(Number),
    Operation { lhs: Number, rhs: Number, op: Op },
    BindingUsage(BindingUsage),
}

impl Expr {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        Self::new_operation(s).or_else(|_| Self::new_number(s))
    }

    fn new_number(s: &str) -> Result<(&str, Self), String> {
        Number::new(s).map(|(s, number)| (s, Self::Number(number)))
    }

    fn new_operation(s: &str) -> Result<(&str, Self), String> {
        let (s, lhs) = Number::new(s)?;
        let (s, _) = extract_whitespace(s, None)?;
        let (s, op) = Op::new(s)?;
        let (s, _) = extract_whitespace(s, None)?;
        let (s, rhs) = Number::new(s)?;
        Ok((s, Self::Operation { lhs, rhs, op }))
    }

    pub fn eval(&self, env: &Env) -> Result<Val, String> {
        match self {
            Self::Number(Number(n)) => Ok(Val::Number(*n)),
            Self::Operation { lhs, rhs, op } => {
                let Number(lhs) = lhs;
                let Number(rhs) = rhs;

                let result = match op {
                    Op::Add => lhs + rhs,
                    Op::Sub => lhs - rhs,
                    Op::Mul => lhs * rhs,
                    Op::Div => lhs / rhs,
                };

                Ok(Val::Number(result))
            }
            Self::BindingUsage(binding_usage) => binding_usage.eval(env),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), Ok(("", Number(123))));
    }

    #[test]
    fn parse_number_as_expr() {
        assert_eq!(Expr::new("456"), Ok(("", Expr::Number(Number(456)))));
    }

    #[test]
    fn parse_add_op() {
        assert_eq!(Op::new("+"), Ok(("", Op::Add)));
    }

    #[test]
    fn parse_sub_op() {
        assert_eq!(Op::new("-"), Ok(("", Op::Sub)));
    }

    #[test]
    fn parse_mul_op() {
        assert_eq!(Op::new("*"), Ok(("", Op::Mul)));
    }

    #[test]
    fn parse_div_op() {
        assert_eq!(Op::new("/"), Ok(("", Op::Div)));
    }

    #[test]
    fn parse_one_plus_two() {
        assert_eq!(
            Expr::new("1+2"),
            Ok((
                "",
                Expr::Operation {
                    lhs: Number(1),
                    rhs: Number(2),
                    op: Op::Add,
                },
            )),
        );
    }

    #[test]
    fn parse_expr_with_whitespace() {
        assert_eq!(
            Expr::new("2 * 2"),
            Ok((
                "",
                Expr::Operation {
                    lhs: Number(2),
                    rhs: Number(2),
                    op: Op::Mul,
                },
            )),
        );
    }

    #[test]
    fn eval_add() {
        let mut env = Env::default();
        env.store_binding("foo".to_string(), Val::Number(10));

        assert_eq!(
            Expr::Operation {
                lhs: Number(10),
                rhs: Number(10),
                op: Op::Add,
            }
            .eval(&Env::default()),
            Ok(Val::Number(20)),
        );
    }

    #[test]
    fn eval_sub() {
        assert_eq!(
            Expr::Operation {
                lhs: Number(1),
                rhs: Number(5),
                op: Op::Sub,
            }
            .eval(&Env::default()),
            Ok(Val::Number(-4)),
        );
    }

    #[test]
    fn eval_mul() {
        assert_eq!(
            Expr::Operation {
                lhs: Number(5),
                rhs: Number(6),
                op: Op::Mul,
            }
            .eval(&Env::default()),
            Ok(Val::Number(30)),
        );
    }

    #[test]
    fn eval_div() {
        assert_eq!(
            Expr::Operation {
                lhs: Number(200),
                rhs: Number(20),
                op: Op::Div,
            }
            .eval(&Env::default()),
            Ok(Val::Number(10)),
        );
    }
}

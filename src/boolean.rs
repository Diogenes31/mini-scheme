use crate::env::Env;
use crate::env::EnvTrait;
use crate::syntax::Atom;
use crate::syntax::Expr;

pub trait LogicOps {
    fn and(&mut self, args: &Vec<Expr>) -> Result<Expr, String>;
    fn or(&mut self, args: &Vec<Expr>) -> Result<Expr, String>;
    fn not(&mut self, args: &Vec<Expr>) -> Result<Expr, String>;
}

impl LogicOps for Env {
    // Returns true if no arguments.
    fn and(&mut self, args: &Vec<Expr>) -> Result<Expr, String> {
        let mut bool_result = true;

        for expr in args.iter() {
            let tree = self.simplify(&expr);

            let result = tree
                .and_then(|x| try_get_bool(&x));

            match result {
                Ok(b) => {
                    bool_result = bool_result && b;
                },
                Err(msg) => {
                    return Err(msg);
                }
            }
        }

        return Ok(Expr::Atom(Atom::Boolean(bool_result)));
    }

    fn or(&mut self, args: &Vec<Expr>) -> Result<Expr, String> {
        let mut bool_result = false;

        for expr in args.iter() {
            let tree = self.simplify(&expr);

            let result = tree
                .and_then(|x| try_get_bool(&x));

            match result {
                Ok(b) => {
                    bool_result = bool_result || b;
                },
                Err(msg) => {
                    return Err(msg);
                }
            }
        }

        return Ok(Expr::Atom(Atom::Boolean(bool_result)));
    }

    fn not(&mut self, args: &Vec<Expr>) -> Result<Expr, String> {
        if args.len() != 1 {
            return Err("Incorrect number of args for 'not' operator.".to_string());
        }
        else {
            let expr = &args[0];
            let tree = self.simplify(expr);

            let result = tree
                .and_then(|x| try_get_bool(&x))
                .map(|x| Expr::Atom(Atom::Boolean(x)));

            return result;
        }
    }
}

fn try_get_bool(expr: &Expr) -> Result<bool, String> {
    match expr {
        Expr::Atom(atom) => match atom {
            Atom::Boolean(b) => Ok(b.to_owned()),
            _ => Err("Cannot perform operation on non-boolean value.".to_string())
        }
        Expr::List(_) => Err("Cannot perform operation on non-boolean value.".to_string())
    }
}

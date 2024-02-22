/// Lambda expression
#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    /// Variable
    Var(String),
    /// Application
    App(Box<Expr>, Box<Expr>),
    /// Abstraction
    Abs(String, Box<Expr>),
}

#[macro_export]
macro_rules! v {
    ($var:expr) => {
        Var($var.to_string())
    };
}

#[macro_export]
macro_rules! app {
    ($e1:expr, $e2:expr) => {
        App(Box::new($e1), Box::new($e2))
    };
}

#[macro_export]
macro_rules! abs {
    ($e1:expr, $e2:expr) => {
        Abs($e1.to_string(), Box::new($e2))
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use Expr::*;

    #[test]
    fn var_macro() {
        assert_eq!(v!("x"), Var("x".to_string()));
    }

    #[test]
    fn app_macro() {
        // x x
        let e1 = app!(v!("x"), v!("x"));
        let e2 = App(Box::new(Var("x".into())), Box::new(Var("x".into())));
        assert_eq!(e1, e2);
    }

    #[test]
    fn abs_macro() {
        let e1 = abs!("x", v!("x"));
        let e2 = Abs("x".to_string(), Box::new(Var("x".to_string())));
        assert_eq!(e1, e2);
    }
}

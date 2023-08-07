#[derive(Debug)]
pub enum Expr<'s> {
    Verbatim(&'s str),
    Ident(&'s str),
    String(&'s str),
}

impl<'s> PartialEq for Expr<'s> {
    fn eq(&self, other: &Self) -> bool {
        use Expr::*;

        match (self, other) {
            (Verbatim(a), Verbatim(b)) => a == b,
            (Verbatim(_), _) => false,

            (Ident(a), Ident(b)) => a == b,
            (Ident(_), _) => false,

            (String(a), String(b)) => a == b,
            (String(_), _) => false,
        }
    }
}

impl<'s> Eq for Expr<'s> {}

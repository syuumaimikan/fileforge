enum Expr {
    Word(String),

    And(Box<Expr>, Box<Expr>),

    Or(Box<Expr>, Box<Expr>),

    Not(Box<Expr>),
}

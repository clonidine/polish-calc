/// Represents an operation in an expression.
#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

/// Represents an expression in an arithmetic expression.
#[derive(Debug, PartialEq)]
pub enum Expr {
    Num(Num),
    Op(Op),
}

/// Represents a numerical value in an arithmetic expression.
#[derive(Debug, PartialEq)]
pub struct Num(f32);

/// Represents a Reverse Polish Notation (RPN) stack.
pub struct Rpn(Vec<f32>);

/// Represents a binary operation in an arithmetic expression.
pub struct BinOp {
    pub op: Op,
    pub lhs: Num,
    pub mhs: Num,
}

impl BinOp {
    /// Executes the binary operation and returns the result.
    pub fn eval(&self) -> f32 {
        match self.op {
            Op::Add => self.lhs.0 + self.mhs.0,
            Op::Sub => self.lhs.0 - self.mhs.0,
            Op::Mul => self.lhs.0 * self.mhs.0,
            Op::Div => self.lhs.0 / self.mhs.0,
        }
    }
}

impl Rpn {
    /// Creates a new empty RPN stack.
    pub fn new() -> Rpn {
        Rpn(Vec::new())
    }

    /// Pushes an expression onto the RPN stack.
    pub fn push(&mut self, expr: Expr) {
        match expr {
            Expr::Num(n) => self.0.push(n.0),
            Expr::Op(op) => {
                let mhs = self.0.pop().unwrap();
                let lhs = self.0.pop().unwrap();

                let bin_op = BinOp {
                    op,
                    lhs: Num(lhs),
                    mhs: Num(mhs),
                };

                self.0.push(bin_op.eval());
            }
        }
    }
}

/// Represents a parser for arithmetic expressions.
pub struct ExprParser<'a> {
    pub input: &'a str,
}

impl<'a> ExprParser<'a> {
    /// Parses the input string into a vector of tokens.
    pub fn parse(&self) -> Vec<Expr> {
        if self.input.is_empty() {
            return vec![Expr::Num(Num(0.0))];
        }

        let mut tokens = vec![];

        for c in self.input.split(' ') {
            match c {
                "+" => tokens.push(Expr::Op(Op::Add)),
                "-" => tokens.push(Expr::Op(Op::Sub)),
                "*" => tokens.push(Expr::Op(Op::Mul)),
                "/" => tokens.push(Expr::Op(Op::Div)),
                _ => tokens.push(Expr::Num(Num(c.parse::<f32>().unwrap()))),
            }
        }

        tokens
    }
}

pub fn calculate(expression: &str) -> f32 {
    let parser = ExprParser { input: expression };

    let tokens = parser.parse();

    let mut rpn = Rpn::new();

    for token in tokens.into_iter() {
        rpn.push(token);
    }

    if rpn.0.len() > 1 {
        return rpn.0[rpn.0.len() - 1];
    }

    rpn.0[0]
}

fn main() {
    println!("{}", calculate("1 2 +"));
    println!("{}", calculate("4 2 /"));
    println!("{}", calculate("1 1 + 2 +"));
    println!("{}", calculate("1 1 + 1 +"));
    println!("{}", calculate("1 1 + 1 + 1 +"));
    println!("{}", calculate("1 1 + 1 + 1 + 1 +"));
    println!("{}", calculate("1 1 + 1 + 1 + 1 + 1 +"));
    println!("{}", calculate("1 1 + 1 + 1 + 1 + 1 + 1 +"));
    println!("{}", calculate("1 1 + 1 + 1 + 1 + 1 + 1 + 1 +"));
    println!("{}", calculate("1 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_work_for_an_empty_string() {
        assert!((calculate("") - 0.0).abs() < 1e-7);
    }

    #[test]
    fn test_should_parse_numbers() {
        assert!((calculate("1 2 3") - 3.0).abs() < 1e-7);
    }

    #[test]
    fn test_should_parse_floats() {
        assert!((calculate("1 2 3.5") - 3.5).abs() < 1e-7);
    }

    #[test]
    fn test_should_support_addition() {
        assert!((calculate("1 3 +") - 4.0).abs() < 1e-7);
    }

    #[test]
    fn test_should_support_multiplication() {
        assert!((calculate("1 3 *") - 3.0).abs() < 1e-7);
    }

    #[test]
    fn test_should_support_subtraction() {
        assert!((calculate("1 3 -") - -2.0).abs() < 1e-7);
    }

    #[test]
    fn test_should_support_division() {
        assert!((calculate("4 2 /") - 2.0).abs() < 1e-7);
    }
}
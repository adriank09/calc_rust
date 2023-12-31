use std::fmt::{Display, Formatter, write};
use std::str::FromStr;

pub struct Calculate {
    pub op: CalcOp,
    pub n1: i32,
    pub n2: i32
}
impl Calculate {
    /// Create an instance of Calculate from env args
    pub fn from_args(args: &[String]) -> Result<Calculate, &'static str> {
        if args.len() < 4 {
            return Err("not enough arguments");
        }
        let op = &args[1];
        let val1 = &args[2];
        let val2 = &args[3];

        // TODO: panics may occur for the following. fix such that they don't panic.
        let o: CalcOp = op.parse().unwrap();
        let v1: i32 = val1.parse().unwrap();
        let v2: i32 = val2.parse().unwrap();

        Ok(Self::new(o, v1, v2))
    }
    /// Creates a new instance of Calculator
    pub fn new(op: CalcOp, n1: i32, n2: i32) -> Calculate {
        Calculate { op, n1, n2 }
    }

    /// Calculates the result.
    pub fn calculate(self: &Self) -> i32 {
        match self.op {
            CalcOp::Add => Calculate::add(self.n1, self.n2),
            CalcOp::Sub => Calculate::sub(self.n1, self.n2),
            CalcOp::Mul => Calculate::mul(self.n1, self.n2),
            CalcOp::Div => Calculate::div(self.n1, self.n2),
        }
    }

    fn add(n1: i32, n2: i32) -> i32 {
        n1 + n2
    }

    fn sub(n1: i32, n2: i32) -> i32 {
        n1 - n2
    }

    fn mul(n1: i32, n2: i32) -> i32 {
        n1 * n2
    }

    fn div(n1: i32, n2: i32) -> i32 {
        n1 / n2
    }

}

/// Calculator operations in enum form.
pub enum CalcOp { Add, Sub, Mul, Div }

/// Displays the string representation of each enum value.
impl Display for CalcOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CalcOp::Add => write!(f, "Addition"),
            CalcOp::Sub => write!(f, "Subtraction"),
            CalcOp::Mul => write!(f, "Multiplication"),
            CalcOp::Div => write!(f, "Division"),
        }
    }
}

/// Converts string to CalcOp enum.
impl FromStr for CalcOp {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let binding = s.to_lowercase();
        let s_lower = binding.as_str();

        match s_lower {
            "add" => Ok(CalcOp::Add),
            "sub" => Ok(CalcOp::Sub),
            "mul" => Ok(CalcOp::Mul),
            "div" => Ok(CalcOp::Div),
            _ => Err(())
        }
    }
}
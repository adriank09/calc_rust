pub mod calculator {
    use std::fmt::{Display, Formatter};
    use std::str::FromStr;

    /// Name of history file which stores the successful calculations done in the past.
    pub const HISTORY_FILE: &str = "history.txt";

    /// calculator.
    pub struct Calculator {
        // Calculation operation
        pub op: CalcOp,
        // First value
        pub n1: i32,
        // Second value
        pub n2: i32
    }

    impl Calculator {
        /// Create an instance of calculator from arguments provided.
        pub fn from_args(args: &[String]) -> Result<Calculator, &'static str> {
            let len = args.len();
            if len < 2 {
                return Err("not enough arguments");
            }

            let op = &args[1];
            let o: CalcOp = op.parse().expect("Expected a valid operation");
            if o != CalcOp::History && len < 4 {
                return Err("not enough arguments");
            }

            let mut v1: i32 = -1;
            let mut v2: i32 = -1;

            if o == CalcOp::History {
                // Skip parsing the two values as they are not expected.
            }
            else {
                let val1 = &args[2];
                let val2 = &args[3];

                v1 = val1.parse().expect("Expected an integer value");
                v2 = val2.parse().expect("Expected an integer value");
            }

            Ok(Self::new(o, v1, v2))
        }

        /// Creates a new instance of calculator
        pub fn new(op: CalcOp, n1: i32, n2: i32) -> Calculator {
            Calculator { op, n1, n2 }
        }

        /// Calculates and returns the result.
        pub fn calculate(self: &Self) -> i32 {
            match self.op {
                CalcOp::Add => Calculator::add(self.n1, self.n2),
                CalcOp::Sub => Calculator::sub(self.n1, self.n2),
                CalcOp::Mul => Calculator::mul(self.n1, self.n2),
                CalcOp::Div => Calculator::div(self.n1, self.n2),
                _ => todo!()
            }
        }

        pub fn get_history_file() { }

        /// Performs addition against the two values.
        fn add(n1: i32, n2: i32) -> i32 {
            n1 + n2
        }

        /// Performs subtraction against the two values.
        fn sub(n1: i32, n2: i32) -> i32 {
            n1 - n2
        }

        /// Performs multiplication against the two values.
        fn mul(n1: i32, n2: i32) -> i32 {
            n1 * n2
        }

        /// Performs division against the two values.
        fn div(n1: i32, n2: i32) -> i32 {
            // To prevent div by zero error.
            if n2 == 0 {
                return 0;
            }
            n1 / n2
        }
    }

    /// calculator operations in enum form.
    #[derive(PartialEq)]
    pub enum CalcOp { Add, Sub, Mul, Div, History }

    /// Displays the string representation of each enum value.
    impl Display for CalcOp {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                CalcOp::Add => write!(f, "Addition"),
                CalcOp::Sub => write!(f, "Subtraction"),
                CalcOp::Mul => write!(f, "Multiplication"),
                CalcOp::Div => write!(f, "Division"),
                CalcOp::History => write!(f, "History")
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
                "hist" => Ok(CalcOp::History),
                _ => Err(())
            }
        }
    }
}

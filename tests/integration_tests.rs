#[cfg(test)]
mod tests {
    use calc::calculator::{CalcOp, Calculator};

    #[test]
    fn addition_is_successful() {
        let calc = Calculator::new(CalcOp::Add, 2, 2);
        let res = calc.calculate();
        assert_eq!(res, 4, "Expected 4, got: {}", res);
    }

    #[test]
    fn subtraction_is_successful() {
        let calc = Calculator::new(CalcOp::Sub, 4, 2);
        let res = calc.calculate();
        assert_eq!(res, 2, "Expected 2, got: {}", res);
    }

    #[test]
    fn multiplication_is_successful() {
        let calc = Calculator::new(CalcOp::Mul, 10, 2);
        let res = calc.calculate();
        assert_eq!(res, 20, "Expected 20, got: {}", res);
    }

    #[test]
    fn division_is_successful() {
        let calc = Calculator::new(CalcOp::Sub, 4, 2);
        let res = calc.calculate();
        assert_eq!(res, 2, "Expected 2, got: {}", res);
    }

    #[test]
    fn division_by_zero_does_not_panic() {
        let calc = Calculator::new(CalcOp::Div, 4, 0);
        let res = calc.calculate();
        assert_eq!(res, 0, "Expected 0, got: {}", res);
    }
}
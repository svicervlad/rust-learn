use std::env::args;

#[derive(Debug)]
pub enum OperationError {
    DivisionByZero,
    UnknownOperation,
}

pub struct Calculator {
    first: f32,
    second: f32,
    option: String,
}

impl Calculator {
    #[allow(dead_code)]
    pub fn new(first: f32, second: f32, option: String) -> Self {
        Self {
            first,
            second,
            option,
        }
    }
    pub fn new_from_args() -> Self {
        let mut args = args();
        let first_sting = args.nth(1).unwrap_or_default();
        let option = args
            .next()
            .unwrap_or_default()
            .chars()
            .next()
            .unwrap_or_default()
            .to_string();
        let second_string = args.next().unwrap_or_default();
        let first = first_sting.parse().unwrap_or_default();
        let second = second_string.parse().unwrap_or_default();
        Self {
            first,
            second,
            option,
        }
    }
    pub fn operate(&self) -> Result<f32, OperationError> {
        let first = self.first;
        let second = self.second;
        let option = self.option.as_str();
        match option {
            "+" => Ok(first + second),
            "-" => Ok(first - second),
            "*" | "x" | "X" => Ok(first * second),
            "/" if second == 0.0 => Err(OperationError::DivisionByZero),
            "/" => Ok(first / second),
            _ => Err(OperationError::UnknownOperation),
        }
    }
}

impl std::fmt::Display for Calculator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.first, self.option, self.second)
    }
}

#[cfg(test)]
mod tests {
    use super::Calculator;
    use super::OperationError;

    #[test]
    fn test_add() {
        let calculator = Calculator::new(2.0, 2.0, "+".to_string());
        let result = calculator.operate().unwrap();
        assert_eq!(result, 4.0);
    }

    #[test]
    fn test_sub() {
        let calculator = Calculator::new(2.0, 2.0, "-".to_string());
        let result = calculator.operate().unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_mul() {
        let calculator = Calculator::new(2.0, 2.0, "*".to_string());
        let result = calculator.operate().unwrap();
        assert_eq!(result, 4.0);
    }

    #[test]
    fn test_div() {
        let calculator = Calculator::new(2.0, 2.0, "/".to_string());
        let result = calculator.operate().unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_div_by_zero() {
        let calculator = Calculator::new(2.0, 0.0, "/".to_string());
        let result = calculator.operate();
        assert!(matches!(result, Err(OperationError::DivisionByZero)));
    }

    #[test]
    fn test_unknown_operation() {
        let calculator = Calculator::new(2.0, 2.0, "a".to_string());
        let result = calculator.operate();
        assert!(matches!(result, Err(OperationError::UnknownOperation)));
    }
}

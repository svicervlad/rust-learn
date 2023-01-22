use std::env::{args};

mod calculator;
use calculator::{Calculator, OperationError};

fn main() {
    let args = args();
    if args.len() < 4 {
        println!("Usage: calculator <first> <operation> <second>");
        println!("Operations: +, -, *, /");
        println!("Example: calculator 2 + 2");
        println!("result: 2 + 2 = 4");
        return;
    }
    let calculator: Calculator = Calculator::new_from_args();
    let result: Result<f32, OperationError> = calculator.operate();
    match result {
        Ok(value) => println!("result: {} = {}", calculator, value),
        Err(error) => println!("Error: {:?}", error)
    }
}

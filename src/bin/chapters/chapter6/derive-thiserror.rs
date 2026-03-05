use thiserror::Error;

#[derive(Debug, Error)]
enum CalculatorError {
    #[error("Division by zero")]
    DivisionByZero,

    #[error("Negative square root: {0}")]
    NegativeSquareRoot(f64),

    #[error("Overflow in calculation")]
    Overflow,
}

type CalcResult<T> = Result<T, CalculatorError>;

fn safe_divide(a: f64, b: f64) -> CalcResult<f64> {
    if b == 0.0 {
        Err(CalculatorError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn safe_sqrt(x: f64) -> CalcResult<f64> {
    if x < 0.0 {
        Err(CalculatorError::NegativeSquareRoot(x))
    } else {
        Ok(x.sqrt())
    }
}

fn over_demo(limit: usize) -> CalcResult<f64> {
    if limit > 5 {
        return Err(CalculatorError::Overflow);
    } else {
        return Ok(20.22);
    }
}

fn calculate(a: f64, b: f64) -> CalcResult<f64> {
    let over = over_demo(7)?;
    println!("{}", over);
    let divided = safe_divide(a, b)?;
    return safe_sqrt(divided);
}

fn main() {
    //   println!("{}", calculate(100.0, 4.0)); // because this return back Result enum we can't print
    println!("{:?}", calculate(100.0, 0.0)); // Err(DivisionByZero)
    println!("{:?}", calculate(-100.0, 1.0)); // Err(NegativeSquareRoot)
}

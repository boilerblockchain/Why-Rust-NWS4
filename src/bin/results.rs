/// `Result` enum with variants `Ok/Err` provides runtime-actionable information
/// on recoverable failure modes.
#[derive(Debug)]
struct DivideByZeroError;

fn divide(numerator: f64, denominator: f64) -> Result<f64, DivideByZeroError> {
  if denominator != 0.0 {
    Ok(numerator / denominator)
  } else {
    Err(DivideByZeroError)
  }
}

fn main() {
  match divide(1.0, 0.0) {
    Ok(quotient) => println!("Quotient: {}", quotient),
    Err(err) => println!("Error: {:?}", err),
  }
}

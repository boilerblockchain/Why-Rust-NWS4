/// Return some number if valid, otherwise none. Built-in `Option` enum with
/// variants `Some/None` forces error handling, and avoids silent runtime bugs.
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
  if denominator != 0.0 {
    Some(numerator / denominator)
  } else {
    None
  }
}

/// `match` statement destructures enum into its variants.
fn main() {
  match divide(1.0, 1.0) {
    Some(quotient) => println!("Quotient: {}", quotient),
    None => println!("Cannot divide by zero"),
  }
}

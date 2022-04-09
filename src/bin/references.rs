/// Any data can have ONE mutable reference, or MANY immutable references.
/// "Borrow-checker" avoids runtime memory bugs which are common in e.g. C++.
#[allow(unused_variables, unused_mut)]
fn main() {
  let s = String::from("Hello");
  // change(&mut s);

  // let r1 = &mut s;
  // let r2 = &mut s;
  // println!("{}, {}", r1, r2);
}

// #[allow(dead_code)]
// fn change(string: &mut String) { string.push_str(", world!"); }

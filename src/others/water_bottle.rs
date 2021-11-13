use proconio::input;
 
fn main() {
  input! {
    a: f64,
    b: f64,
    x: f64
  }

  if a * a * b <= 2f64 * x {
    println!("{}", (2f64 * (a * b - (x / a)) / (a * a)).atan().to_degrees());
  } else {
    println!("{}", (b / (2f64 * x / (b * a))).atan().to_degrees());
  }
}
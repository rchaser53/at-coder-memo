use proconio::input;
 
fn main() {
  input! { L: f64 }
  let val = L / 3f64;
  println!("{}", val * val * val);
}
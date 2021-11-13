use proconio::input;
 
fn main() {
  input! {
    A: f64,
    B: usize,
  }
  
  let original = B * 10;
  for v in 0..10 {
    if ((original + v) as f64 * 0.08f64).floor() == A {
      println!("{}", original + v);
      return
    }
  }
  println!("{}", -1);
}

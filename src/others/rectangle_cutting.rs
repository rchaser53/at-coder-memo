use proconio::input;

fn main() {
  input! {
    w: f64,
    h: f64,
    x: f64,
    y: f64
  }
  println!("{}", w * h / 2f64);
  if w / 2f64 == x  && h / 2f64 == y {
    println!("1");  
  } else {
    println!("0");
  }
}
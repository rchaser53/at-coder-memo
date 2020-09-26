use proconio::input;
use proconio::marker::*;

fn main() {
  input! {
    mut a: usize,
    mut b: usize,
    c: usize,
    mut x: usize,
    mut y: usize,
  }
  
  if x < y {
    let temp = x;
    x = y;
    y = temp;
    let temp = a;
    a = b;
    b = temp;
  }

  let pa = x * (2 * c);
  let pb = y * (2 * c) + (x - y) * a;
  let pc = x * a + y * b;
  
  let mut min = std::cmp::min(pa, pb);
  min = std::cmp::min(min, pc);
  println!("{}", min);
}

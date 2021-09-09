use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    s:isize
  }

  let x1 = 10isize.pow(9);
  let x2 = 1;

  if s == 10isize.pow(18) {
    println!("0 0 {} {} {} {}", x1, 0, x2, x1);
    return
  }

  let y2 = s / x1 + 1;
  let y1 = x1 * y2 - s;
  println!("0 0 {} {} {} {}", x1, y1, x2, y2);
}

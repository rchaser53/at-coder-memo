use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    n:usize,
    mut x:usize,
    mut vals:[usize;n]
  }
  vals.sort();
  let mut result = 0;
  for i in 0..n-1  {
    let v = vals[i];
    if v <= x {
      x -= v;
      result += 1;
    } else {
      break
    }
  }
  if x == vals[n-1] {
    result += 1;
  }

  println!("{}", result);
}
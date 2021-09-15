use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
	  n:usize,
    vals:[f64;n]
  }
  let tot = vals.iter().sum::<f64>();
  let mut count = 0f64;
  for v in vals {
    if 0f64 < v {
      count += 1f64;
    }
  }
  println!("{}", (tot / count).ceil());
}

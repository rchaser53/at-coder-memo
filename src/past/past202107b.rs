use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    vals:[isize;3]
  }

  let v = std::cmp::min(vals[1] * vals[2], vals[0]) as f64;
  println!("{}", v / vals[1] as f64);
}
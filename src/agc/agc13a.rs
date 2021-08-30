use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    n:usize,
    vals:[usize;n]
  }
  let mut result = 1;
  let mut last = vals[0];
  let mut p = 0;
  for i in 1..n {
    let v = vals[i];
    if p == 0 {
      if v < last {
        p = 1;
      } else if last < v {
        p = 2;
      }
    } else if p == 1 {
      if last < v {
        result += 1;
        p = 0;
      }
    } else {
      if last > v {
        result += 1;
        p = 0;
      }
    }
    last = v;
  }
  println!("{}", result);
}
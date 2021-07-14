/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
    a:[Usize1;n],
    b:[usize;n],
    c:[usize;n-1],
  }
  let mut result = 0;

  let mut last = 1_000;
  for ti in a {
    result += b[ti];
    if ti == last + 1 {
      result += c[last];
    }
    last = ti;
  }
  println!("{}", result);
}

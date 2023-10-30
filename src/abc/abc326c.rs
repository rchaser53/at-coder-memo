/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Reverse;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    mut a:[usize;n]
  }

  a.sort();
  let mut result = 0;
  let mut ri = 0;
  for li in 0..n {
    while ri < n && a[ri] - a[li] < m {
      ri += 1;
    }
    result = result.max(ri-li);

    if n <= ri {
      break
    }
  }

  println!("{}", result);
}
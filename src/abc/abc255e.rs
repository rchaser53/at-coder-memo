/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    s:[isize;n-1],
    x:[isize;m]
  }

  let mut b = vec![0;n+5];
  for i in 1..n {
    b[i] = s[i-1] - b[i-1];
  }

  let mut map = HashMap::new();
  for i in 1..=n {
    for j in 1..=m {
      let mut c = x[j-1] - b[i-1];
      if i % 2 == 0 { c *= -1; }
      *map.entry(c).or_insert(0) += 1;
    }
  }

  let mut result = 0;
  for (_, v) in map {
    result = std::cmp::max(result, v);
  }

  println!("{}", result);
}
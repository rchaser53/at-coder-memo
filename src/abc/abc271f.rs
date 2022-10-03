/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    rows:[[usize;n];n]
  }

  let limit = 1 << n-1;
  let mut result = 0usize;
  let mut memo = vec![vec![HashMap::new();n];n];
  for i in 0..limit {
    let mut temp = rows[0][0];
    let mut r = 0;
    let mut c = 0;
    for j in 0..n-1 {
      if i >> j & 1 == 1 {
        r += 1;
      } else {
        c += 1;
      }
      temp ^= rows[r][c];
    }
    *memo[r][c].entry(temp).or_insert(0) += 1;
  }

  for i in 0..limit {
    let mut temp = rows[n-1][n-1];
    let mut r = n-1;
    let mut c = n-1;
    for j in 0..n-1 {
      if i >> j & 1 == 1 {
        r -= 1;
      } else {
        c -= 1;
      }
      temp ^= rows[r][c];
    }
    temp ^= rows[r][c];

    if let Some(num) = memo[r][c].get(&temp) {
      result += num;
    }
  }
  println!("{}", result);
}
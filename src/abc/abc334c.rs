/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    k:usize,
    a:[usize;k]
  }

  if k == 1 {
    println!("0");
    return
  }

  let mut tot = 0;
  for i in (0..k-1).step_by(2) {
    tot += a[i+1] - a[i];
  }

  if k % 2 == 0 {
    println!("{}", tot);
    return
  }

  let mut memo = vec![0;k];
  let mut temp = 0;
  for i in (1..k).step_by(2).rev() {
    memo[i+1] = temp;
    temp += a[i+1] - a[i];
    memo[i] = temp;
  }
  memo[0] = temp;

  let mut result = tot.min(memo[0]);
  let mut temp = 0;
  for i in 1..k {
    if i % 2 == 0 {
      result = result.min(temp+memo[i]);
    } else {
      let v = temp + a[i+1] - a[i-1] + memo[(i+2).min(k-1)];
      result = result.min(v);
      temp += a[i] - a[i-1];
    }
  }

  println!("{}", result);
}
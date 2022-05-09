/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    x:i32,
    vals: [[i32;m+1];n]
  }

  let nl = 1 << n;
  let inf = 1_000_000_000;
  let mut result = inf;
  for i in 0..nl {
    let mut score = vec![0;m];
    let mut cost = 0;
    for j in 0..n {
      if i >> j & 1 == 1 {
        cost += vals[j][0];
        for k in 1..=m {
          score[k-1] += vals[j][k];
        }
      }
    }

    let mut success = true;
    for v in score {
      if v < x {
        success = false;
        break
      }
    }
    if success {
      result = std::cmp::min(result, cost);
    }
  }

  if result == inf {
    println!("-1");
  } else {
    println!("{}", result);
  }
}
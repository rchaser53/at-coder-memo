/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
    m:usize,
    vals:[(Usize1,Usize1);m]
  }

  let mut memo = vec![(1,false);n];
  memo[0].1 = true;

  for (from, to) in vals {
    if memo[from].1 {
      memo[to].1 = true;
    }
    memo[from].0 -= 1;
    memo[to].0 += 1;
    if memo[from].0 == 0 {
      memo[from].1 = false;
    }
  }

  let mut count = 0usize;
  for (v, flag) in memo {
    if flag && 0 < v {
      count += 1;
    }
  }

  println!("{}", count);
}

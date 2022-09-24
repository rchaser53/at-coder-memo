/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;

fn main() { 
  input! { 
    n:usize,
    a:[usize;n]
  }
  // 0:選ばない 1:選ぶ
  let mut memo = vec![(0,0);n+1];
  for i in 0..n {
    memo[i+1].0 = memo[i+1].0.max(memo[i].0).max(memo[i].1);
    memo[i+1].1 = memo[i+1].1.max(memo[i].0+a[i]);
  }

  println!("{}", memo[n].0.max(memo[n].1));
}
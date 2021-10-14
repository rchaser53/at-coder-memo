use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input!{
    n:usize,
    vals:[usize;n]
  }

  let mut memo = vec![0,0];
  for v in vals {
    memo[v%2] += 1;
  }

  if memo[1] % 2 == 1 {
    println!("NO");
  } else {
    println!("YES");
  }
}
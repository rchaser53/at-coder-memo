use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    n:usize,
    p:usize,
    vals:[usize;n]
  }

  let a = 100;
  let limit = a * n + 1;
  let mut memo = vec![0usize;limit];
  memo[0] = 1;
  for v in vals {
    let mut new_memo = memo.clone();
    for i in 0..limit {
      let nv = i + v;
      if limit <= nv { break }
      new_memo[nv] += memo[i];
    }
    memo = new_memo;
  }

  let mut result = 0usize;
  for i in 0..limit {
    if i % 2 == p {
      result += memo[i];
    }
  }
  println!("{}", result);
  
}
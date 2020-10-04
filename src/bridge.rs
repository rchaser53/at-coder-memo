#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::{HashMap, VecDeque};
 
fn main() {
  input!{
    n: usize,
    m: usize,
    mut ab: [(Usize1, Usize1);m]
  }
  let mut memo: Vec<Vec<usize>> = vec![vec![];n];
  for (a, b) in ab {
    memo[a].push(b);
    memo[b].push(a);
  }
  
  let mut flag = true;
  let mut count = 0;
  while flag {
    flag = false;
    for i in 0..n {
      if memo[i].len() == 1 {
        flag = true;
        count += 1;
        let target = memo[i][0];
        memo[target].retain(|&x| x != i);
        memo[i].clear();
      }
    }
  }
  println!("{}", count);
}

#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;

#[fastout]
fn main() {
  input!{
    n: usize,
    mut vals: [[Usize1;n-1];n]
  }
  
  let mut count = 0;
  let mut memo = vec![(0, false);n];
  loop {
    let mut flag = true;
    for i in 0..n {
      memo[i].1 = false;
    }

    for i in 0..n {
      let from = memo[i];
      if from.0 == n-1 || from.1 { continue }

      let ti = vals[i][from.0];
      let to = memo[ti];
      if to.0 == n-1 || to.1 { continue }
      
      if vals[ti][to.0] != i { continue }
      memo[i] = (memo[i].0+1, true);
      memo[ti] = (memo[ti].0+1, true);
      flag = false;
    }
    count += 1;
    if flag { break }
  }
  for (v, _) in memo {
    if v != n-1 {
      println!("-1");
      return
    }
  }
  println!("{}", count-1);
}
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

struct Helper {
  dp: Vec<isize>,
  memo: Vec<Vec<usize>>
}

impl Helper {
  fn culc(&mut self, ci:usize) -> isize {
    if 0 < self.dp[ci] {
      return self.dp[ci]
    }
    
    self.dp[ci] = 0;
    let mut temp = 0;
    for i in 0..self.memo[ci].len() {
      let v = self.culc(self.memo[ci][i]) + 1;
      temp = std::cmp::max(temp, v);
    }
    self.dp[ci] = std::cmp::max(self.dp[ci], temp);
    temp
  }
}

const MOD:usize = 1_000_000_007;
fn main() {
  input!{
    n:usize,
    m:usize,
    vals:[(Usize1,Usize1);m]
  }
  
  let mut memo = vec![vec![];n];
  for (from, to) in vals {
    memo[from].push(to);
  }
  let mut helper = Helper { memo, dp:vec![-1;n] }; 
  
  for i in 0..n {
    helper.culc(i);
  }
  println!("{}", helper.dp.into_iter().max().unwrap());
}
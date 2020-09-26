#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::VecDeque;

fn helper(
  flags: &mut Vec<Vec<bool>>,
  dp: &mut Vec<Vec<isize>>,
  vals: &Vec<isize>,
  l: usize,
  r: usize,
) -> isize {
  if flags[l][r] { return dp[l][r] }
  flags[l][r] = true;
  
  if l == r {
    dp[l][r] = vals[l];
    return dp[l][r];
  }
  
  dp[l][r] = std::cmp::max(
    vals[l] - helper(flags, dp, vals, l+1, r),
    vals[r] - helper(flags, dp, vals, l, r-1)
  );
  
  dp[l][r]
}

fn main() {
  input!{
    n: usize,
    vals: [isize;n]
  }
  
  let mut flags: Vec<Vec<bool>> = vec![vec![false;n];n];
  let mut dp: Vec<Vec<isize>> = vec![vec![0;n];n];
  println!("{}", helper(&mut flags, &mut dp, &vals, 0, n-1));
}
/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

const MOD:usize = 1_000_000_007;

pub fn main(
) {
  input! {
    n:usize,
    m:usize,
    vals:[(Usize1, Usize1, usize);n]
  }
  
  let mut nv = 0;
  let max = vals.iter().map(|v| v.2).sum::<usize>();
  let mut memo = vec![(0, 0);m+1];

  for (l, r, v) in vals {
    memo[l].0 += v;
    memo[r+1].1 += v;
  }

  let mut result = 0;
  for i in 0..m {
    nv -= memo[i].1;
    nv += memo[i].0;
    result = std::cmp::max(result, max - nv);
  }
  
  println!("{}", result);
}

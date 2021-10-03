use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use std::cmp::Reverse;

const MOD:usize = 998244353;

fn helper(vals:&[usize], memo:&Vec<usize>) -> usize {
  if vals.len() == 1 {
    return 0
  }
  let hn = vals.len() / 2;
  let mut evals = vals.iter().cloned().enumerate().collect::<Vec<_>>();
  evals.sort_by_key(|e| e.1);
  let (l, r) = vals.split_at(hn);
  let mut result = helper(l, memo) + helper(r, memo);
  let mut tot = 0usize;
  for (x, _) in evals {
    if x < hn {
      tot = (tot + memo[hn-1-x]) % MOD;
    } else {
      result += tot * memo[x-hn] % MOD;
    }
  }
  result
}
fn main() {
  input!{
    n:usize,
    vals: [usize;n]
  }
  let mut memo = vec![1usize;n+1];
  for i in 1..=n {
    memo[i] = 2 * memo[i-1] % MOD;
  }
  println!("{}", helper(&vals, &memo) % MOD);
}
/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;

const MOD:usize = 998244353;
fn main() {
  input! {
    n:usize,
    a:[usize;n],
    b:[usize;n]
  }

  let limit = 3001;
  let mut memo = vec![0;limit+1];
  for i in a[0]..=b[0] {
    memo[i] += 1;
  }

  for i in 1..n {
    let av = a[i];
    let bv = b[i];
    let mut new_memo = vec![0;limit+1];

    let mut prefix_sum = vec![0;limit+2];
    for j in 0..=limit {
      prefix_sum[j+1] += (prefix_sum[j] + memo[j]) % MOD;
    }
    for j in av..=bv {
      new_memo[j] = prefix_sum[j+1];
    }

    memo = new_memo;
  }

  let mut result = 0;
  for v in memo {
    result += v;
    result %= MOD;
  }

  println!("{}", result);
}
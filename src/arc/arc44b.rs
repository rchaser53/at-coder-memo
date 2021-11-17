/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

const MOD:usize = 1_000_000_007;

// n^pを計算するやつ
fn repeat_square(n:usize, p:usize) -> usize {
  if p == 0 {
    1
  } else if p % 2 == 0 {
    repeat_square(n, p/2).pow(2) % MOD 
  } else {
    n * repeat_square(n, p-1) % MOD
  }
}

fn main() {
  input! {
    n:usize,
    vals:[usize;n]
  }

  if vals[0] != 0 {
    println!("0");
    return
  }

  let mut map = HashMap::new();
  for v in vals {
    *map.entry(v).or_insert(0) += 1;
  }

  let mut memo = vec![];
  for (key, v) in map {
    memo.push((key, v));
  }
  memo.sort_by(|a,b| a.0.cmp(&b.0));
  if memo[0].0 != 0 || memo[0].1 != 1 {
    println!("0");
    return
  }

  for i in 0..memo.len() {
    if memo[i].0 != i {
      println!("0");
      return
    }
  }

  let mut dp = vec![0;n+1];
  for i in 1..=n {
    dp[i] += dp[i-1] + i;
  }

  let mut result = 1usize;
  for i in 1..memo.len() {
    let lv = memo[i-1].1;
    let cv = memo[i].1;

    // 同じレベルに対しての線の引き方
    let p = dp[cv - 1];
    let v = repeat_square(2, p) % MOD;
    result *= v;
    result %= MOD;

    // 1つ上のレベルに対しての線の引き方
    let bv = (MOD + repeat_square(2, lv) - 1) % MOD;
    let v = repeat_square(bv, cv) % MOD;
    
    result *= v;
    result %= MOD;
  }
  println!("{}", result);
}
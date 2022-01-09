/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

const MOD:usize = 998244353;

// nCk choose
// n種類同じ文字がある文字列の並び替え
struct Binom {
  fact: Vec<usize>,
  fact_inv: Vec<usize>,
}
impl Binom {
  fn init(n:usize) -> Binom {
    let mut fact = vec![0;n+1];
    let mut inv = vec![0;n+1];
    let mut fact_inv = vec![0;n+1];
    fact[0] = 1;
    fact[1] = 1;
    inv[1] = 1;
    fact_inv[0] = 1;
    fact_inv[1] = 1;
    for i in 2..=n {
      fact[i] = fact[i-1] * i % MOD;
      inv[i] = MOD - inv[MOD % i] * (MOD / i) % MOD;
      fact_inv[i] = fact_inv[i-1] * inv[i] % MOD;
    }
    Binom {
      fact,
      fact_inv,
    }
  }

  fn get(&self, n:usize, k:usize) -> usize {
    self.fact[n] * self.fact_inv[k] % MOD * self.fact_inv[n-k] % MOD
  }
}

fn main() {
  input! {
    mut s: Chars,
  }

  let n = s.len();
  let mut memo = vec![0usize;26];
  for c in s {
    let i = (c as u8 - 'a' as u8) as usize;
    memo[i] += 1;
  }
  let binom = Binom::init(n);

  let mut dp = vec![0;n+1];
  dp[0] = 1;
  // n種類 同じ文字 がある 文字列の並び替えは
  // 1種類ずつ増やすことで対応できる
  for i in 0..26 {
    let mut next = vec![0;n+1];
    for j in 0..=n {
      let limit = std::cmp::min(j, memo[i]);
      for k in 0..=limit {
        next[j] += dp[j-k] * binom.get(j,k);
        next[j] %= MOD;
      }
    }
    dp = next;
  }
  let mut result = 0;
  for i in 1..=n {
    result += dp[i];
    result %= MOD;
  }
  println!("{}", result);
}
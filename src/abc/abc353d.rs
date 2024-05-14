use proconio::input;
use proconio::marker::*;

const MOD:usize = 998244353;
fn main() {
  input! {
    n:usize,
    a:[usize;n],
  }

  let mut dp = vec![0;11];
  dp[a[n-1].to_string().len()] += 1;
  let mut result = 0;
  result += a[n-1] * (n-1) % MOD;
  for i in (0..n-1).rev() {
    let v = a[i];
    let v1 = v * i % MOD;
    result += v1;
    result %= MOD;
    for j in 0..=10 {
      let mut bv = v;
      bv *= dp[j];
      bv %= MOD;
      bv *= 10usize.pow(j as u32) % MOD;
      bv %= MOD;
      result += bv;
      result %= MOD;
    }
    dp[v.to_string().len()] += 1;
  }
  println!("{}", result);
}

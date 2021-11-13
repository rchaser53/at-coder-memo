use proconio::input;
 
const MOD:isize = 998244353;
fn main() {
  input! {
    n: usize,
    k: usize,
    vals: [(usize, usize);k]
  }
 
  let mut dp: Vec<isize> = vec![0;n+1];
  let mut dp_sum: Vec<isize> = vec![0;n+1];
  dp[1] = 1;
  dp_sum[1] = 1;
  for i in 2..=n {
    for ii in 0..k {
      let (l, r) = vals[ii];
      let ri = if i <= l {
        continue
      } else {
        i - l
      };
      let li = if i <= r {
        0
      } else {
        i - r - 1
      };
      
      let mut v = (dp_sum[ri] - dp_sum[li]) % MOD;
      if v < 0 {
        v += MOD;
      }
      dp[i] += v;
      dp[i] %= MOD;
    }
    dp_sum[i] = (dp_sum[i-1] + dp[i]) % MOD;
  }
  println!("{}", dp[n]);
}
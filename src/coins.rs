use proconio::input;

fn main() {
  input!{
    n: usize,
    vals: [f64;n]
  }
  
  let half = (n / 2) + 1;
  let mut dp = vec![vec![0f64;n+1];n+1];
  
  dp[0][0] = 1f64;
  for i in 1..=n {
    for ii in 0..i+1 {
      if 0 < ii {
        dp[i][ii] = dp[i-1][ii] * (1f64 - vals[i-1]) + dp[i-1][ii-1] * vals[i-1];
      } else {
        dp[i][ii] += dp[i-1][ii] * (1f64 - vals[i-1]);
      }
    }
  }

  let mut result = 0f64;
  for i in half..=n {
    result += dp[n][i];
  }
  println!("{}", result);
}

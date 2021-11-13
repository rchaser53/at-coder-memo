use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
  input! {
    s: usize,
  }
  
  let mut dp: Vec<usize> = vec![0;s+1];  
  if s < 3 {
    println!("0");
    return
  }
  
  if s < 6 {
    println!("1");
    return
  }
  
  dp[3] = 1;
  dp[4] = 1;
  dp[5] = 1;
  for i in 6..=s {
    dp[i] = (dp[i-1] + dp[i-3]) % MOD;
  }
  
  println!("{}", dp[s]);
}
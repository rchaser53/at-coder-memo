use proconio::input;
 
fn main() {
  input! {
    n: usize,
    vals: [(usize, usize, usize);n]
  }
  
  let mut dp: Vec<(usize, usize, usize)> = vec![(0,0,0);n];
  dp[0].0 = vals[0].0;
  dp[0].1 = vals[0].1;
  dp[0].2 = vals[0].2;
  
  for i in 1..n {
    dp[i].0 = vals[i].0 + std::cmp::max(dp[i-1].1, dp[i-1].2);
    dp[i].1 = vals[i].1 + std::cmp::max(dp[i-1].0, dp[i-1].2);
    dp[i].2 = vals[i].2 + std::cmp::max(dp[i-1].1, dp[i-1].0);
  }
  
  let mut result = std::cmp::max(dp[n-1].0, dp[n-1].1);
  result = std::cmp::max(result , dp[n-1].2);
  
  println!("{}", result);
}
use proconio::input;
use std::collections::HashSet;
 
fn main() {
  input! {
    n: usize,
    w: usize,
    mut vals: [(usize, usize);n]
  }
  
  let mut dp: Vec<Vec<usize>> = vec![vec![0;w+1];n+1];
  for i in 0..n {
    for ii in 0..=w {
      let (weight, value) = vals[i];
      if weight <= ii {
        dp[i+1][ii] = std::cmp::max(dp[i+1][ii], dp[i][ii-weight] + value);
      }
      
      dp[i+1][ii] = std::cmp::max(dp[i+1][ii], dp[i][ii]);
    }
  }
  
  println!("{}", dp[n][w]);
}
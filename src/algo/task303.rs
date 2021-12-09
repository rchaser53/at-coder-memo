/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;

fn readln<T: std::str::FromStr>() -> T {
  let mut tmp = String::new();
  std::io::stdin().read_line(&mut tmp).ok();
  tmp.trim().parse().ok().unwrap()
}
fn readvec<T: std::str::FromStr>() -> Vec<T> {
  readln::<String>()
      .split_whitespace()
      .map(|x| x.parse().ok().unwrap())
      .collect()
}

fn main() {
  let n:usize = readln();
  let vals:Vec<usize> = readvec();

  let max = 1_000_000_000usize;
  let mut dp = vec![max;n+1];

  dp[0] = 0;
  dp[1] = 0;

  for i in 2..=n {
    let v = vals[i-1];
    dp[i] = dp[i].min(dp[i-1]+v).min(dp[i-2]+2*v);
  }
  println!("{}", dp[n]);
}
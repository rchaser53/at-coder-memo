use proconio::input;
 
fn main() {
  input! {
    n: usize,
    hs: [i64; n],
  }
  let mut dp = vec![0i64;n];
  dp[0] = 0;
  dp[1] = (hs[1] - hs[0]).abs();
  
  for i in 2..n {
    dp[i] = std::cmp::min(
      (hs[i] - hs[i-1]).abs() + dp[i-1],
      (hs[i] - hs[i-2]).abs() + dp[i-2],
    );
  }
  println!("{}", dp[n-1]);
}
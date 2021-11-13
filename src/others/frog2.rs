use proconio::input;
 
fn main() {
  input! {
    n: usize,
    k: usize,
    vals: [isize;n]
  }
  
  if n-1 <= k {
    println!("{}", (vals[n-1] - vals[0]).abs());
    return
  };
  
  let mut dp: Vec<isize> = vec![isize::max_value();n];
  for i in 0..=k {
    dp[i] = (vals[i] - vals[0]).abs();
  }

  for i in k+1..n {    
    for ii in 1..=k {
      dp[i] = std::cmp::min(
        (vals[i]-vals[i-ii]).abs() + dp[i-ii],
        dp[i]
      );
    }
  }
  
  println!("{}", dp[n-1]);
}
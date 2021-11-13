use proconio::{input, fastout};

#[fastout]
fn main() {
  input! {
    n: usize,
    k: usize,
    vals: [isize;n]
  }
  
  let mut memo: Vec<isize> = vec![0;50];
  for &v in &vals {
    for i in 0..50 {
      if (v >> i) & 1 > 0 {
        memo[i] += 1;
      }
    }
  }

  let n = n as isize;
  let mut dp: Vec<(isize, isize)> = vec![(-1,0); 51];
  for i in (0..50).rev() {
    if dp[i+1].0 >= 0 {
      dp[i].0 = dp[i+1].0 + (std::cmp::max(memo[i], n-memo[i]) << i);
    }
    if (k >> i) & 1 > 0 {
      dp[i].0 = std::cmp::max(dp[i].0, dp[i+1].1 + (memo[i] << i));
      dp[i].1 = dp[i+1].1 + ((n - memo[i]) << i);
    } else {
      dp[i].1 = dp[i+1].1 + (memo[i] << i);
    }
  }
  println!("{}", std::cmp::max(dp[0].0, dp[0].1));
}
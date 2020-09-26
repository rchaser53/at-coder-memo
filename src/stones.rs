use proconio::input;
use proconio::marker::*;

fn main() {
  input!{
    n: usize,
    k: usize,
    vals: [usize;n]
  }
  let mut dp = vec![false;k+1];
  for i in 1..=k {
    for ii in 0..n {
      let v = vals[ii];
      if v <= i && !dp[i-v] {
        dp[i] = true;
      }
    }
  }

  if dp[k] {
    println!("First");
  } else {
    println!("Second");
  }
}

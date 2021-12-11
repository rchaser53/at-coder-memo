/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
use std::cmp::Reverse;

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
  let vals = readvec();
  let n = vals.len();
  let mut memo = vec![vec![0usize;n];n];

  for i in 0..n {
    memo[0][i] = vals[i];
  }

  for i in 1..n {
    for j in 0..n {
      if 0 < j {
        memo[i][j] += memo[i-1][j-1];
      }
      memo[i][j] += memo[i-1][j];
      if j < n-1 {
        memo[i][j] += memo[i-1][j+1];
      }
    }
  }

  println!("{}", memo[n-1][n-1]);
}
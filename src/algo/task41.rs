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
  let n = readln();
  let mut dict:Vec<Vec<usize>> = vec![];
  for _ in 0..n {
    dict.push(readvec());
  }

  let mut memo = vec![vec![0usize;3];n];
  for i in 0..3 {
    memo[0][i] = dict[0][i];
  }

  for i in 1..n {
    for j in 0..3 {
      for k in 0..3 {
        if j == k { continue }
        memo[i][k] = std::cmp::max(memo[i][k], memo[i-1][j] + dict[i][k]);
      }
    }
  }

  let mut max = 0;
  for i in 0..3 {
    max = std::cmp::max(max, memo[n-1][i]);
  }
  println!("{}", max);
}
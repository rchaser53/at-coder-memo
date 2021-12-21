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
  let mut dict = vec![];

  for _ in 0..n {
    let mut v:Vec<usize> = readvec();
    v.reverse();
    dict.push(v);
  }

  let mut memo = vec![vec![1_000_000_000usize;n];n];
  memo[0][0] = dict[0][0];

  for i in 0..n {
    for j in 0..n {
      if 0 < j {
        memo[i][j] = std::cmp::min(memo[i][j], memo[i][j-1] + dict[i][j]);
      }
      if 0 < i {
        memo[i][j] = std::cmp::min(memo[i][j], memo[i-1][j] + dict[i][j]);
      }
    }
  }
  println!("{}", memo[n-1][n-1]);
}
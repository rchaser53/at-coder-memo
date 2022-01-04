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
fn read_chars() -> Vec<char> {
  let mut tmp = String::new();
  std::io::stdin().read_line(&mut tmp).ok();
  let tmp:String = tmp.trim().parse().ok().unwrap();
  tmp.chars().into_iter().collect::<Vec<char>>()
}

fn main() {
  let dict:Vec<usize> = readvec();
  let (n, m) = (dict[0], dict[1]);

  let mut c:Vec<Vec<usize>> = vec![];
  for _ in 0..n {
    c.push(readvec());
  }

  let inf = 1_000_000_000;
  let mut memo = vec![vec![inf;m];n];
  memo[0][0] = c[0][0];
  for i in 1..m {
    memo[0][i] = c[0][i] + memo[0][i-1];
  }

  for i in 0..n {
    for j in 0..m {
      if 0 < i {
        memo[i][j] = std::cmp::min(memo[i][j], memo[i-1][j] + c[i][j]);
      }
      if 0 < j {
        memo[i][j] = std::cmp::min(memo[i][j], memo[i][j-1] + c[i][j]);
      }
      if 0 < i && 0 < j {
        memo[i][j] = std::cmp::min(memo[i][j], memo[i-1][j-1] + c[i][j]);
      }
    }
  }

  println!("{}", memo[n-1][m-1]);
}
/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/

use std::cmp::Ordering;
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
fn read_chars() -> Vec<char> {
  let mut tmp = String::new();
  std::io::stdin().read_line(&mut tmp).ok();
  let tmp:String = tmp.trim().parse().ok().unwrap();
  tmp.chars().into_iter().collect::<Vec<char>>()
}

fn main() {
  let nm:Vec<usize> = readvec();
  let n = nm[0];
  let m = nm[1];
  let mut c:Vec<Vec<usize>> = vec![];
  for _ in 0..n {
    c.push(readvec());
  }

  let inf = 1_000_000_000usize;
  let mut memo = vec![vec![inf;m+1];n+1];
  memo[0][0] = 0;
  for i in 0..n {
    for j in 0..m {
      let v = c[i][j];
      memo[i+1][j+1] = memo[i+1][j+1].min(memo[i][j]+v).min(memo[i][j+1]+v).min(memo[i+1][j]+v);   
    }
  }

  println!("{}", memo[n][m]);
}
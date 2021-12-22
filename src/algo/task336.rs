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
  let dict:Vec<usize> = readvec();
  let (n, m) = (dict[0], dict[1]);
  let dict:Vec<usize> = readvec();
  let mut memo = vec![vec![false;m];n];


  memo[0][0] = true;
  
  for i in 0..n-1 {
    for j in 0..m {
      if !memo[i][j] { continue }
      memo[i+1][j] = true;
      let ni = j + dict[i];
      if m <= ni { continue }
      memo[i+1][ni] = true;
    }
  }

  let mut result = 0;
  for j in 0..m {
    if memo[n-1][j] {
      result += 1;
    }
  }

  println!("{}", result);
}
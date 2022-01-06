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
  let t:usize = readln();
  let mut g:Vec<Vec<usize>> = vec![];

  for _ in 0..t {
    g.push(readvec());
  }

  let mut memo = vec![0;t+2];
  for i in 1..=t+1 {
    for j in 0..i {
      for k in j+1..i {
        memo[i] = std::cmp::max(memo[i], memo[j]+g[j][k-1]);
      }
    }
  }
  println!("{}", memo[t+1]);
}
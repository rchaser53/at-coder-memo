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
  let s = read_chars();
  let t = read_chars();

  let mut memo = vec![vec![0;t.len()+1];s.len()+1];
  for i in 0..s.len() {
    let sc = s[i];
    for j in 0..t.len() {
      let tc = t[j];
      if sc == tc {
        memo[i+1][j+1] = std::cmp::max(memo[i+1][j+1], memo[i][j]+1);
      } else {
        memo[i+1][j+1] = memo[i+1][j+1]
                          .max(memo[i+1][j])
                          .max(memo[i][j+1]);
      }
    }
  }

  println!("{}", memo[s.len()][t.len()]);
}
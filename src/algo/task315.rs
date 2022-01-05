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

// レーベンシュタイン距離 (diff コマンド)
fn main() {
  let s = read_chars();
  let t = read_chars();

  let inf = 1_000_000_000;
  let limit = 1010;
  let mut memo = vec![vec![inf;limit];limit];
  memo[1][1] = 0;

  for i in 0..=s.len() {
    for j in 0..=t.len() {
      if i == 0 && j == 0 { continue }
      if 1 <= i && 1 <= j {
        if s[i-1] == t[j-1] {
          memo[i+1][j+1] = std::cmp::min(memo[i+1][j+1], memo[i][j]);
        } else {
          memo[i+1][j+1] = std::cmp::min(memo[i+1][j+1], memo[i][j]+1);
        }
      }

      if 1 <= i {
        memo[i+1][j+1] = std::cmp::min(memo[i+1][j+1], memo[i][j+1] + 1);
      }
      if 1 <= j {
        memo[i+1][j+1] = std::cmp::min(memo[i+1][j+1], memo[i+1][j] + 1);
      }
    }
  }

  println!("{}", memo[s.len()+1][t.len()+1]);
}
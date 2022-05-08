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

  let n = s.len();
  let m = t.len();

  let inf = 1_000_000_000;
  let mut memo = vec![vec![inf;m+1];n+1];
  memo[0][0] = 0;
  // insertとdeleteがあるので(0,1)と(1,0)をケアしないと駄目
  for i in 0..=n {
    for j in 0..=m {
      if 0 < i {
        memo[i][j] = memo[i][j].min(memo[i-1][j]+1);
      }

      if 0 < j {
        memo[i][j] = memo[i][j].min(memo[i][j-1]+1);
      }

      if 0 < i && 0 < j {
        if s[i-1] == t[j-1] {
            memo[i][j] = memo[i][j].min(memo[i-1][j-1]);
        } else {
            memo[i][j] = memo[i][j].min(memo[i-1][j-1]+1)
        }
      }
    }
  }
  println!("{}", memo[n][m]); 
}
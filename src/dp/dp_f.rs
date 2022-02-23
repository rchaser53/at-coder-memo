/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::*;
use std::rc::*;
use std::cell::*;

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
fn readchars() -> Vec<char> {
  let mut tmp = String::new();
  std::io::stdin().read_line(&mut tmp).ok();
  let tmp:String = tmp.trim().parse().ok().unwrap();
  tmp.chars().into_iter().collect::<Vec<char>>()
}


fn main() {
  let s = readchars();
  let sn = s.len();
  let t = readchars();
  let tn = t.len();
  let mut dp = vec![vec![0;tn+1];sn+1];
  for i in 0..sn {
    for j in 0..tn {
      if s[i] == t[j] {
        dp[i+1][j+1] = std::cmp::max(dp[i][j]+1, dp[i+1][j+1]);
      } else {
        dp[i+1][j+1] = dp[i+1][j+1]
                        .max(dp[i][j+1])
                        .max(dp[i+1][j]);
      }
    }
  }

  let mut result = format!("");
  let mut i = sn;
  let mut j = tn;
  while 0 < i && 0 < j {
    if dp[i-1][j] == dp[i][j] {
      i -= 1;
    } else if dp[i][j-1] == dp[i][j] {
      j -= 1;
    } else {
      result = format!("{}{}", s[i-1], result);
      i -= 1;
      j -= 1;
    }    
  }
  println!("{}", result);
}
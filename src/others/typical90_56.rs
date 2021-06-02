/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::{input};
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
    mut s:usize,
    vals:[(usize,usize);n]
  }
  
  let mut result = vec![];
  for limit in (1..n).rev() {
    let mut dp = vec![vec![false;s+1];limit+1];
    dp[0][0] = true;
    for i in 0..limit {
      let (l, r) = vals[i];
      for j in 0..=s {
        if !dp[i][j] { continue }
        let lv = l + j;
        let rv = r + j;
        
        if lv < s {
          dp[i+1][lv] = true;
        }
  
        if rv < s {
          dp[i+1][rv] = true;
        }
      }
    }

    let (l, r) = vals[limit];
    if l <= s && dp[limit][s-l] {
      result.push(String::from("A"));
      s -= l;
    } else if r <= s && dp[limit][s-r] {
      result.push(String::from("B"));
      s -= r;
    } else {
      println!("Impossible");
      return
    }
  }

  if s == vals[0].0 {
    result.push(String::from("A"));
  } else if s == vals[0].1 {
    result.push(String::from("B"));
  } else {
    println!("Impossible");
    return
  }

  result.reverse();
  println!("{}", result.into_iter().collect::<String>());
}
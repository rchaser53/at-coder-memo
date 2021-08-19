/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    s:Chars
  }
  let n = s.len();
  let s = s.into_iter().map(|v| (v as u8 - 'a' as u8 ) as usize).collect::<Vec<usize>>();

  let mut memo = vec![vec![vec![false;26];n+1];n+1];
  for i in 0..n {
    let v = s[i];
    memo[0][i][v] = true;
  }

  for i in 0..n-1 {
    let ti = n - i;
    for j in 0..26 {
      let mut flag = true;
      for k in 0..ti {
        if memo[i][k][j] {
          memo[i+1][k][j] = true;
          if 0 < k {
            memo[i+1][k-1][j] = true;
          }
        } else {
          flag = false;
        }
      }

      if flag {
        println!("{}", i);
        return
      }
    }
  }
  println!("{}", n - 1);
}

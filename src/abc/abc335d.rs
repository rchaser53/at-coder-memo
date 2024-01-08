/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
  }
  
  let mut memo = vec![vec![0;n];n];
  let p = [(1,0),(0,1),(-1,0),(0,-1)];
  let mut pi = 0;
  let default = 1_000_000_000;
  memo[n/2][n/2] = default;

  let mut now = (n/2, n/2-1);
  for i in 1..n*n {
    memo[now.0][now.1] = i;

    let ni = (pi + 1) % 4;
    let next = ((now.0 as i32 + p[ni].0) as usize, (now.1 as i32 + p[ni].1) as usize);
    if memo[next.0][next.1] == 0 {
      pi = ni;
    }
    now = ((now.0 as i32 + p[pi].0) as usize, (now.1 as i32 + p[pi].1) as usize);
  }

  for i in 0..n {
    println!("{}", memo[i].iter().map(|v| if *v == default {
      format!("T")
    } else {
      format!("{}", v)
    }).collect::<Vec<String>>().join(" "));
  }
}
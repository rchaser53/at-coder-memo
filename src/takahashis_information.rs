#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::VecDeque;

fn main() {
  let n = 3;
  input!{
    vals: [[isize;n];n]
  }
 
  let mut memo: Vec<isize> = vec![0;n];
  for v in -100..=100 {
    for i in 0..n {
      memo[i] = vals[0][i] - v;
    }
    
    let mut flag = true;
    for i in 1..n {
      let vv = vals[i][0] - memo[0];
      for ii in 1..n {
        if vv + memo[ii] != vals[i][ii] {
          flag = false;
        }
      }
    }
    
    if flag {
      println!("Yes");
      return
    }
  }

  println!("No");
}

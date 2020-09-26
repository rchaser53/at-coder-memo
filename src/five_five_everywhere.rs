#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::HashSet;

fn main() {
  input! {
    n: usize
  }
  
  let mut memo: Vec<usize> = vec![0;n];
  let mut base = 11;
  memo[0] = 11;
  for i in 1..n {
    loop {
      base += 10;
      let mut v = 3;
      let mut succeed = true;
      while v * v <= base {
        if base % v == 0 {
          succeed = false;
          break;
        }
        v += 1;
      }
    
      if succeed {
        memo[i] = base;
        break
      }
    }
  }
  
  println!("{}", memo
    .into_iter()
    .map(|v| v.to_string())
    .collect::<Vec<String>>()
    .join(" ")
  );
}

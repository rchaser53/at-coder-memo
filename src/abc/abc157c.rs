/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
    m:usize,
    vals:[(Usize1,usize);m]
  }

  let mut memo = vec![None;n];

  for (i, v) in vals {
    if let Some(rv) = memo[i] {
      if rv != v {
        println!("-1");
        return
      }
    } else {
      memo[i] = Some(v);
    }
  }
  
  if let Some(v) = memo[0] {
    if v == 0 && n != 1 {
      println!("-1");
      return
    }
  } else {
    if n == 1 {
      println!("0");
      return
    }
    memo[0] = Some(1);
  }
  for i in 1..n {
    if memo[i].is_none() {
      memo[i] = Some(0);
    }
  }
  println!(
    "{}",
    memo.iter().map(|v| v.unwrap().to_string()).collect::<String>().parse::<usize>().unwrap()
  );
}

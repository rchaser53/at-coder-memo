/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::{input};
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
    vals:[isize;n]
  }
  
  let max = vals.iter().sum::<isize>();
  println!("{}", max);
  let mut culculative = vec![0;n+1];
  for i in 0..n {
    culculative[i+1] = culculative[i] + vals[i];
  }
  
  if n == 1 {
    println!("{}", max);
    return
  } else if n == 2 {
    println!("{}", (vals[0] - vals[1]).abs());
    return
  }
  let mut min = 1_000_000_000_000isize;

  for first in 0..n {
    let a = culculative[first+1];
    let b = culculative[n] - culculative[first+1];
    min = std::cmp::min(min, (a-b).abs());

    for second in 0..first {
      let a = culculative[second+1];
      let b = culculative[first+1] - culculative[second+1];
      let c = culculative[n] - culculative[first+1];

      if a + b > c && b + c > a && c + a > b {
        println!("0");
        return
      }

      min = std::cmp::min(min, (a+b-c).abs());
      min = std::cmp::min(min, (b+c-a).abs());
      min = std::cmp::min(min, (c+a-b).abs());
    }
  }

  println!("{}", min);
}
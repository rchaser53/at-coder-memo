/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

fn main() {
  input! {
    n:usize,
    vals:[(char,String);n]
  }

  let mut memo = vec![0;6];
  for i in 0..n {
    let (p, v) = &vals[i];
    if v == "WA" { continue }
    let j = (*p as u8 - 'A' as u8) as usize;
    if memo[j] == 0 {
      memo[j] = i + 1;
    }
  }

  for v in memo {
    println!("{}", v);
  }
}
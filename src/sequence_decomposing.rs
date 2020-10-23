#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;

#[fastout]
fn main() {
  input! {
    n: usize,
    vals: [isize;n]
  }
  
  let mut memo: VecDeque<isize> = VecDeque::new();
  memo.push_back(vals[0]);
  for i in 1..n {
    if vals[i] <= memo[0] {
      memo.push_front(vals[i]);
    } else {
      let mut ng = -1;
      let mut ok = memo.len() as isize;
      while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        if memo.get(mid as usize).unwrap() >= &vals[i] {
          ok = mid;
        } else {
          ng = mid;
        }
      }
      memo[ng as usize] = vals[i];
    }
  }

  println!("{}", memo.len());
}
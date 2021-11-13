/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input! {
    k:usize,
    t:usize,
    mut vals:[usize;t]
  }

  vals.sort();
  vals.reverse();
  let mut vals = vals.into_iter().enumerate().collect::<VecDeque<(usize, usize)>>();
  
  let mut memo = vec![0;k];
  let mut ci = 0;
  while ci < k {
    memo[ci] = vals[0].0;
    vals[0].1 -= 1;
    if vals[0].1 == 0 {
      vals.pop_front();
    }
    ci += 2;
  }

  let mut ci = 1;
  while ci < k {
    if vals[0].0 == memo[ci-1] && 1 < vals.len() {
      memo[ci] = vals[1].0;
      vals[1].1 -= 1;
      if vals[1].1 == 0 {
        vals.remove(1);
      }
    } else {
      memo[ci] = vals[0].0;
      vals[0].1 -= 1;
      if vals[0].1 == 0 {
        vals.pop_front();
      }
    }
    ci += 2;
  }

  let mut result = 0;
  for i in 1..k {
    if memo[i-1] == memo[i] {
      result += 1;
    }
  }
  println!("{}", result);
}
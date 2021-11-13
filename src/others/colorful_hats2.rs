#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::{HashMap, HashSet, VecDeque};

const MOD:usize = 1_000_000_007;

#[fastout]
fn main() {
  input!{
    n: usize,
    vals: [usize;n],
  }
  
  let mut memo = [0,0,0];
  let mut result = 1;
  for v in vals {
    let mut c = 0;
    let mut should_update = true;
    for i in 0..3 {
      if v == memo[i] {
        c += 1;
        if should_update {
          memo[i] += 1;
          should_update = false;
        }
      }
    }
    result *= c;
    result %= MOD;
  }
  
  println!("{}", result);
}
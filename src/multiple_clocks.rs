#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::{HashMap, HashSet, VecDeque};

fn gcv(a:u128, b:u128) -> u128 {
  if b == 0 {
    a
  } else {
    gcv(b, a % b)
  }
}

fn lcm(a: u128, b:u128) -> u128 {
  a * b / gcv(a, b)
}

#[fastout]
fn main() {
  input!{
    n: usize,
    mut vals: [u128;n]
  }
  
  vals.sort();
  let mut result = vals[0];
  for i in 1..n {
    result = lcm(result, vals[i]);
  }
  
  println!("{}", result);  
}

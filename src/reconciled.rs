#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::{HashMap, HashSet, VecDeque};

const MOD:usize = 1_000_000_007;

#[fastout]
fn main() {
  input!{
    n: usize,
    m: usize
  }
  
  let mut a = 1;
  for i in 2..=n {
    a *= i;
    a %= MOD;
  }
  
  let mut b = 1;
  for i in 2..=m {
    b *= i;
    b %= MOD;
  }
  
  if m == n {
    println!("{}", (a * b) % MOD * 2 % MOD); 
  } else if (m as isize - n as isize).abs() == 1 {
    println!("{}", (a * b) % MOD);
  } else {
    println!("0");
  }
}
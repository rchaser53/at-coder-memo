#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::mem;

const MOD:usize = 1_000_000_007;
#[fastout]
fn main() {
  input!{
    n: usize,
    k: usize,
    mut vals: [(Usize1, Usize1);n-1]
  }
  
  let mut list = vec![vec![];n];
  for (from, to) in vals {
    list[from].push(to);
    list[to].push(from);
  }
  
  let mut ans = k;
  if k < list[0].len() {
    println!("0");
    return
  }
  
  for i in 1..=list[0].len() {
    ans *= k - i;
    ans %= MOD;
  }
  
  for i in 1..n {
    let child = list[i].len() - 1;
    if k - 2 < child {
      println!("0");
      return
    }
    
    for j in 0..child {
      ans *= k - 2 - j;
      ans %= MOD;
    }
  }
  println!("{}", ans);
}
#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use petgraph::unionfind::UnionFind;

const MOD:usize = 998244353;

fn culc(a:usize) -> usize {
  if a % 2 == 0 {
    ((a+1) % MOD) * (a/2) % MOD
  } else {
    ((a+1)/2 % MOD) * a % MOD
  }
}

#[fastout]
fn main() {
  input!{
    a: usize,
    b: usize,
    c: usize,
  }

  println!("{}", culc(c) * culc(b) % MOD * culc(a) % MOD);
}
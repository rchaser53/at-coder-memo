#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

const MOD:usize = 1_000_000_007;
fn main() {
  input!{
    a:usize,
    b:usize,
    c:usize
  }
  
  println!("{}", a * b % MOD * c % MOD);
}
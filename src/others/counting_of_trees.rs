#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use petgraph::unionfind::UnionFind;

const MOD:usize = 998244353;

#[fastout]
fn main() {
  input!{
    n: usize,
    mut vals: [usize;n]
  }

  if vals[0] != 0 ||
    (vals.len() > 1 && vals[1] == 0)
  {
    println!("0");
    return
  }
  
  vals.sort();
  let mut i = 0;
  let mut memo = vec![1];
  for ii in 1..n {
    let node = vals[ii];
    if node == i + 1 {
      memo.push(1);
      i += 1;
    } else if node == i && node > 0 {
      memo[i] += 1;
    } else {
      println!("0");
      return;
    }
  }
  
  let mut result = 1;
  let mut last = 1;
  for i in 1..memo.len() {
    let v = memo[i];
    for i in 0..v {
      result *= last;
      result %= MOD;
    }
    last = v;
  }
  println!("{}", result);
}
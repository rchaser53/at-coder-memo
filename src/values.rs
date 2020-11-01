#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use petgraph::unionfind::UnionFind;

#[fastout]
fn main() {
  input!{
    n: usize,
    m: usize,
    originals: [i64; n],
    converteds: [i64; n],
    nodes:[(Usize1, Usize1);m]
  }
  
  let mut tree: UnionFind<usize> = UnionFind::new(n);
  for (from, to) in nodes {
    tree.union(from, to);
  }
  
  let mut sum_originals = vec![0;n];
  let mut sum_converteds = vec![0;n];
  
  for i in 0..n {
    sum_originals[tree.find(i)] += originals[i];
    sum_converteds[tree.find(i)] += converteds[i];
  }
  
  for i in 0..n {
    if sum_originals[i] != sum_converteds[i] {
      println!("No");
      return
    }
  }
  println!("Yes");
}
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
    vals: [(Usize1, Usize1);m]
  }
  
  let mut tree: UnionFind<usize> = UnionFind::new(n);
  for (from, to) in vals {
    tree.union(from, to);
  }
  
  let mut set = HashSet::new();
  for i in 0..n {
    set.insert(tree.find(i));
  }
  println!("{}", set.len()-1);
}
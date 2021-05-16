#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;
 
#[fastout]
fn main() {
  input!{
    n:usize,
    m:usize,
    vals:[(Usize1,Usize1,usize);m]
  }
  
  let mut tree: UnionFind<usize> = UnionFind::new(n);
  for i in 0..m {
    tree.union(vals[i].0, vals[i].1);
    tree.union(vals[i].1, vals[i].0);
  }
  
  let mut set = HashSet::new();
  for i in 0..n {
    set.insert(tree.find(i));
  }
  println!("{}", set.len());
}
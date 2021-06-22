/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use proconio::{input};
use proconio::marker::*;
use std::collections::*;
use petgraph::unionfind::UnionFind;

pub fn main(
) {
  input! {
    n:usize,
    vals:[usize;n]
  }

  let mut tree: UnionFind<usize> = UnionFind::new(2*10usize.pow(5) + 10);
  for i in 0..n/2 {
    tree.union(vals[i], vals[n-i-1]);
  }

  let mut map = HashMap::new();
  for v in vals {
    let entry_set = map.entry(tree.find(v)).or_insert(HashSet::new());
    entry_set.insert(v);
  }

  let mut result = 0usize;
  for (_, set) in map {
    result += set.len() - 1;
  }
  println!("{}", result);
}
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
    vals:[(Usize1, Usize1);n]
  }

  let mut tree: UnionFind<usize> = UnionFind::new(400010);
  for &(from, to) in &vals {
    tree.union(from, to);
  }
  
  let mut map = HashMap::new();

  for (l, r) in vals {
    let parent = tree.find(l);
    let mut entry = map.entry(parent).or_insert((0, HashSet::new()));
    entry.0 += 1;
    entry.1.insert(l);
    entry.1.insert(r);
  }

  let mut result = 0;
  for (_, (edge_num, nodes)) in map {
    let len = nodes.len();
    if len <= edge_num {
      result += len;
    } else {
      result += len - 1;
    }
  }
  println!("{}", result);
}
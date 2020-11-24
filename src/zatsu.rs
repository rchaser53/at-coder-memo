#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    n: usize,
    vals: [usize;n]
  }
  
  let mut set = HashSet::new();
  for v in vals.iter() {
    set.insert(v);
  }
  
  let mut keys = set
    .into_iter()
    .map(|v| *v)
    .collect::<Vec<usize>>();
  keys.sort();
  let mut map = HashMap::new();
  for (i, key) in keys.into_iter().enumerate() {
    map.insert(key, i);
  }
  
  for v in vals {
    println!("{}", map.get(&v).unwrap());
  }
}
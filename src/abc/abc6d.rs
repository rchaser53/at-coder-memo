#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use std::ops::Bound::Included;

fn main() {
  input!{
    n: usize,
    vals: [Usize1;n]
  }
  
  let limit = 1_000_000;
  let mut btmap = BTreeMap::new();
  btmap.insert(limit, 0);
  let mut max = 0;
  for i in (0..n).rev() {
    let mut temp = 0;
    let v = vals[i];
    btmap.insert(v, 0);
    for (_, &value) in btmap.range((Included(&v), Included(&limit))) {
      temp = std::cmp::max(temp, value);
    }
    btmap.insert(v, temp+1);
    max = std::cmp::max(max, temp+1);
  }
  println!("{}", n-max);
}
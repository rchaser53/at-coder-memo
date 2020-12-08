#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    mut n: isize,
    ng1: isize,
    ng2: isize,
    ng3: isize,
  }
  
  let mut set = HashSet::new();
  set.insert(ng1);
  set.insert(ng2);
  set.insert(ng3);
  
  if set.contains(&n) {
    println!("NO");
    return
  }
  
  let mut sorted = vec![ng1, ng2, ng3];
  sorted.sort();
  if sorted[0] + 1 == sorted[1]
    && sorted[1] + 1 == sorted[2]
    && sorted[2] < n {
    println!("NO");
    return
  }
    
  let mut count = 1;
  loop {
    if !set.contains(&(n - 3)) {
      n -= 3;
    } else if !set.contains(&(n - 2)) {
      n -= 2;
    } else {
      n -= 1;
    }
    
    if n <= 0 {
      break
    }
    
    count += 1;
  }
  
  if 100 < count {
    println!("NO");
  } else {
    println!("YES");
  }
}
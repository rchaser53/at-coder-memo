#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::HashMap;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::heap_recursive;

fn main() {
  input!{
    n:usize,
    m:usize,
    r:usize,
    mut lists:[Usize1;r],
    vals:[(Usize1,Usize1,usize);m]
  }
  
  let g = UnGraph::<usize, usize, usize>::from_edges(&vals);  
  let mut map = HashMap::new();
  for i in 0..r {
    let si = lists[i];
    let res = dijkstra(&g, si.into(), Some(n.into()), |e| *e.weight());
    let mut temp = vec![0;n];
    for j in 0..n {
      temp[j] = *res.get(&NodeIndex::new(j)).unwrap();
    }
    map.insert(si, temp);
  }

  let mut result = 1_000_000_000_000_000_000usize;
  heap_recursive(&mut lists, |p| {
    let mut temp = 0;
    for i in 1..r {
      let from = p[i-1];
      let to = p[i];
      let dict = map.get(&from).unwrap();
      temp += dict[to];
    }
    result = result.min(temp);
  });
  println!("{}", result);
}

fn sol2() {
  input!{
    n:usize,
    m:usize,
    r:usize,
    mut lists:[Usize1;r],
    vals:[(Usize1,Usize1,usize);m]
  }
  
  let mut memo = vec![vec![1_000_000_000_000usize;n];n];
  for i in 0..m {
    let (l, r, v) = vals[i];
    memo[l][r] = v;
    memo[r][l] = v;
  }
  for i in 0..n {
    memo[i][i] = 0;
  }
  
  for i in 0..n {
    for j in 0..n {
      for k in 0..n {
        memo[j][k] = std::cmp::min(
          memo[j][k], memo[j][i] + memo[i][k]
        );
      }
    }
  }
 
  let mut result = 1_000_000_000_000_000_000usize;
  heap_recursive(&mut lists, |p| {
    let mut temp = 0;
    for i in 1..r {
      temp += memo[p[i-1]][p[i]];
    }
    result = result.min(temp);
  });
  println!("{}", result);
}
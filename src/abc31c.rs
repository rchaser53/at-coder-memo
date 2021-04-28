#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n:usize,
    vals:[isize;n]
  }
  
  let default = -500000;
  let mut max = default;
  for i in 0..n {
    let mut best = default;
    let mut ti = 0;
    let mut pair = (0,0);
    for j in 0..n {
      if i == j { continue }
      let (start, goal) = if i < j {
        (i, j)
      } else {
        (j, i)
      };
      
      let mut temp = 0;
      for k in start..=goal {
        let v = k - start;
        if v % 2 == 1 {
          temp += vals[k];
        }
      }
      if best < temp {
        best = temp;
        pair = (start, goal);
        ti = j;
      } else if temp == best {
        if j < ti {
          ti = j;
          pair = (start, goal);          
        }
      }
    }
        
    let mut temp = 0;
    for j in pair.0..=pair.1 {
      let v = j - pair.0;
      if v % 2 == 0 {
        temp += vals[j];
      }
    }
    max = std::cmp::max(max, temp);
  }
  println!("{}", max);
}

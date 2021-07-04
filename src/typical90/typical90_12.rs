#![allow(unused_imports)]
use proconio::{input};
use proconio::marker::*;
use itertools::Itertools;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};
use std::collections::*;

const MOD:usize = 1_000_000_007;
const MAX: usize = 1000;

pub fn main(
) {
  input! {
    h:usize,
    w:usize,
    q:usize,
  }

  let mut rows = vec![vec![false;w];h];
  let n = h * w;
  let mut tree: UnionFind<usize> = UnionFind::new(n);
  for _ in 0..q {
    input!{
      t:usize
    }
    
    if t == 1 {
      input!{
        y:Usize1, x:Usize1,
      }
      rows[y][x] = true;
      let i = y * w + x;
      if 0 < x && rows[y][x-1] {
        tree.union(i, i-1);
      }
      if x < w-1 && rows[y][x+1] {
        tree.union(i, i+1);
      }
      if 0 < y && rows[y-1][x] {
        tree.union(i, i-w);
      }
      if y < h-1 && rows[y+1][x] {
        tree.union(i, i+w);
      }
      
    } else {
      input!{
        y1:Usize1, x1:Usize1, 
        y2:Usize1, x2:Usize1, 
      }
      let ti1 = y1 * w + x1;
      let ti2 = y2 * w + x2;
      if tree.find(ti1) == tree.find(ti2)
        && rows[y1][x1]
        && rows[y2][x2]
      {
        println!("Yes");
      } else {
        println!("No");
      }
    }
  }
}

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
    n: usize,
    mut rows: [Chars;n]
  }
  
  let column = 2*n-1;
  
  for r in (1..n).rev() {
    for c in 0..column {
      if rows[r][c] == 'X' {
        if 0 < c && rows[r-1][c-1] == '#' {
          rows[r-1][c-1] = 'X';
        }
        
        if rows[r-1][c] == '#' {
          rows[r-1][c] = 'X';
        }
        
        if c < column-1 && rows[r-1][c+1] == '#' {
          rows[r-1][c+1] = 'X';
        }
      }
    }
  }
  
  for row in rows {
    println!("{}", row.into_iter().map(|v| v.to_string()).collect::<String>());
  }
}
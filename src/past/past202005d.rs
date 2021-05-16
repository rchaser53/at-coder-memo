#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};

const MOD:usize = 1_000_000_000;

fn main() {
  input!{
    n: usize,
    defs: [Chars;5]
  }
  
  let dict = vec![
    // 0
    vec![
      vec![true, true, true],
      vec![true, false, true],
      vec![true, false, true],
      vec![true, false, true],
      vec![true, true, true],
    ],
    // 1
    vec![
      vec![false, true, false],
      vec![true, true, false],
      vec![false, true, false],
      vec![false, true, false],
      vec![true, true, true],
    ],
    // 2
    vec![
      vec![true, true, true],
      vec![false, false, true],
      vec![true, true, true],
      vec![true, false, false],
      vec![true, true, true],
    ],
    // 3
    vec![
      vec![true, true, true],
      vec![false, false, true],
      vec![true, true, true],
      vec![false, false, true],
      vec![true, true, true],
    ],
    // 4
    vec![
      vec![true, false, true],
      vec![true, false, true],
      vec![true, true, true],
      vec![false, false, true],
      vec![false, false, true],
    ],
    // 5
    vec![
      vec![true, true, true],
      vec![true, false, false],
      vec![true, true, true],
      vec![false, false, true],
      vec![true, true, true],
    ],
    // 6
    vec![
      vec![true, true, true],
      vec![true, false, false],
      vec![true, true, true],
      vec![true, false, true],
      vec![true, true, true],
    ],
    // 7
    vec![
      vec![true, true, true],
      vec![false, false, true],
      vec![false, false, true],
      vec![false, false, true],
      vec![false, false, true],
    ],
    // 8
    vec![
      vec![true, true, true],
      vec![true, false, true],
      vec![true, true, true],
      vec![true, false, true],
      vec![true, true, true],
    ],
    // 9
    vec![
      vec![true, true, true],
      vec![true, false, true],
      vec![true, true, true],
      vec![false, false, true],
      vec![true, true, true],
    ],
  ];
  
  let mut result = vec![];
  for i in 0..n {
    for ii in 0..10 {
      let mut success = true;
      for r in 0..5 {
        if !success { break }
        for iii in (1+4*i)..(4+4*i) {
          let v = defs[r][iii] == '#';
          if dict[ii][r][iii % 4 - 1] != v {
            success = false;
            break
          }
        }
      }
      if success {
        result.push(ii.to_string());
      }
    }
  }
  
  println!("{}", result.into_iter().collect::<String>());
}

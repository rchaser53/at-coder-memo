#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use permutohedron::{Heap, heap_recursive};

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    h: usize,
    w: usize,
    wn: usize,
    hn: usize
  }
  let mut result = vec![vec![String::from("");w];h];
  let mut last = 0;
  for i in 0..h {
    for ii in 0..w {
      if i < h-hn {
        if ii < wn {
          result[i][ii] = String::from("1");
        } else {
          result[i][ii] = String::from("0");
        }
      } else {
        if wn <= ii {
          result[i][ii] = String::from("1");
        } else {
          result[i][ii] = String::from("0");
        }
      }
    }
  }
  
  for row in result {
    println!("{}", row.into_iter().collect::<String>());
  }
}
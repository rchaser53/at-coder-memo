#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use itertools::Itertools;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};
use std::collections::*;
use superslice::Ext;

const MOD:usize = 1_000_000_007;

#[fastout]
fn main() {
  input! {
    s: Chars
  }
  
  let len = s.len();
  let half = len/2;
  let mut eq_num = 0;
  for i in 0..half {
    if s[i] == s[len-i-1] {
      eq_num += 1; 
    }
  }
  
  if eq_num + 1 < half {
    println!("{}", len * 25);
  } else if eq_num + 1 == half {
    println!("{}", len * 25 - 2);
  } else if eq_num == half {
    if len % 2 == 1 {
      println!("{}", (len - 1) * 25);
    } else {
      println!("{}", len * 25);
    }
  } else {
    panic!("nya-n");
  }
}
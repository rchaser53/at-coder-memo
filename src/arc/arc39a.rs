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

fn main() {
  input!{
    a: Chars,
    b: Chars
  }
  
  let mut aa = a.into_iter()
           .map(|v| v.to_string().parse::<isize>().unwrap())
           .collect::<Vec<isize>>();
  let a = aa[0] * 100 + aa[1] * 10 + aa[2];
  let mut bb = b.into_iter()
           .map(|v| v.to_string().parse::<isize>().unwrap())
           .collect::<Vec<isize>>();
  let b = bb[0] * 100 + bb[1] * 10 + bb[2];
  let mut max = -999;
  
  max = std::cmp::max(max, 900      +10*aa[1]+aa[2]-b);
  max = std::cmp::max(max, 100*aa[0]+10*9    +aa[2]-b);
  max = std::cmp::max(max, 100*aa[0]+10*aa[1]+9    -b);
  
  max = std::cmp::max(max, a - (100      + 10*bb[1]+bb[2]));
  max = std::cmp::max(max, a - (100*bb[0]          +bb[2]));
  max = std::cmp::max(max, a - (100*bb[0]+ 10*bb[1]      ));
  
  println!("{}", max);
}
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

const MOD:usize = 998244353;
const MAX:usize = 200_000;

fn culc(
  vals: &Vec<isize>,
  mut last: isize,
  mut count: isize
) -> isize {
  for i in 1..vals.len() {
    let v = vals[i];
    let nv = last + v;
    if 0 < last {
      if 0 <= nv {
        last = -1;
        count += nv.abs() + 1;
      } else {
        last = nv;
      }
    } else {
      if nv <= 0 {
        last = 1;
        count += nv.abs() + 1;
      } else {
        last = nv;
      }
    }
  }
  count
}

fn main() {
  input!{
    n:usize,
    vals:[isize;n]
  }
  
  if vals[0] == 0 {    
    println!("{}", std::cmp::min(
        culc(&vals, 1, 1),
        culc(&vals, -1, 1)
      )
    );
  } else if vals[0] < 0 {
    println!("{}", std::cmp::min(
        culc(&vals, 1, vals[0].abs()+1),
        culc(&vals, vals[0], 0)
      )
    );
  } else {
    println!("{}", std::cmp::min(
        culc(&vals, vals[0], 0),
        culc(&vals, -1, vals[0].abs()+1)
      )
    );
  }
}
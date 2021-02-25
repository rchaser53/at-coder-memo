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
  input!{
    n: usize,
    r: usize,
    mut s: Chars
  }
  
  let mut left = 0;
  for i in 0..n {
    if s[i] == '.' {
      left += 1;
    }
  }

  let mut count = 0;
  let mut i = 0;
  while i < n && 0 < left {
    if left <= r {
      let mut temp = 0;
      for ii in i..i+r {
        if n <= ii {
          println!("{}", count+1);
          return
        }

        if s[ii] == '.' {
          temp += 1;
        }
      }
      if temp == left {
        println!("{}", count+1);
        return
      }
    }
    
    if s[i] == '.' {
      count += 1;
      for ii in i..i+r {
        if n-1 <= ii {
          println!("{}", count);
          return
        }

        if s[ii] == '.' {
          s[ii] = 'o';
          left -= 1;
        }
      }
    }
    i += 1;
    count += 1;
  }
  println!("{}", count);
}
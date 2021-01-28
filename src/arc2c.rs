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
use std::cmp::*;

const MOD:usize = 1_000_000_007;

#[fastout]
fn main() {
  input!{
    n: usize,
    s: Chars
  }
  
  if n == 1 {
    println!("1");
    return
  }
  
  let dict = vec!['A', 'B', 'X', 'Y'];
  let mut min = n;
  
  for i in 0..dict.len() {
    for ii in 0..dict.len() {
      for iii in 0..dict.len() {
        for iiii in 0..dict.len() {
          let base1 = (dict[i], dict[ii]);
          let base2 = (dict[iii], dict[iiii]);

          let mut ni = 0;
          let mut count = 0;
          while ni < n-1 {
            if (s[ni] == base1.0 && s[ni+1] == base1.1)
              || (s[ni] == base2.0 && s[ni+1] == base2.1) {
              ni += 1;
            }
            ni += 1;
            count += 1;
          }
          if ni == n-1 {
            ni += 1;
            count += 1;
          }
          
          min = std::cmp::min(min, count);
        }
      }
    }
  }
  println!("{}", min);
}
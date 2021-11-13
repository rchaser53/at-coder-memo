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
const MAX: usize = 1000;
 
fn main() {
  input!{
    n: usize
  }
  
  if n == 1 {
    println!("Yes");
    println!("2");
    println!("1 1");
    println!("1 1");
    return
  }
  
  let mut i = 2;
  while i * (i+1) <= 3 * n {
    if i * (i+1) == 2 * n {
      println!("Yes");
      println!("{}", i+1);
      let mut result = vec![vec![0;i];i+1];
      let mut start = 1;
      for ii in 0..i {
        for iii in ii..i {
          let v = start+iii;
          result[ii][iii] = v;
          result[iii+1][ii] = v;
        }
        start += i-ii-1;
      }
      
      for i in 0..i+1 {
        let len = result[i].len();
        let v = result[i].iter().map(|v| v.to_string()).join(" ");
        println!("{} {}", len, v);
      }
      
      return
    } else {
      i += 1;
    }
  }
  println!("No");
}
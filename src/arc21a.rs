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
    rows:[[usize;4];4]
  }
  
  for i in 0..4 {
    for ii in 0..4 {
      if 0 < i && rows[i-1][ii] == rows[i][ii] {
        println!("CONTINUE");
        return
      }
      
      if i < 3 && rows[i+1][ii] == rows[i][ii] {
        println!("CONTINUE");
        return
      }
      
      if ii < 3 && rows[i][ii+1] == rows[i][ii] {
        println!("CONTINUE");
        return
      }
      
      if 0 < ii && rows[i][ii-1] == rows[i][ii] {
        println!("CONTINUE");
        return
      }
    }
  }  
  println!("GAMEOVER");  
}

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
  
  let mut stack = vec![];
  for i in 1..=555555 {
    let v = i.to_string().chars().into_iter().collect::<Vec<char>>();
    let first = v[0];
    let mut success = true;
    for ii in 1..v.len() {
      if v[ii] != first {
        success = false;
        break
      }
    }
    if success {
      stack.push(i); 
    }
  }
  println!("{}", stack[n-1]); 
}
#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    s: Chars,
    t: Chars,
  }
  
  let mut flag = true;
  let sv = if s[1] == 'F' {
    flag = true;
    s[0].to_string().parse::<isize>().unwrap()
  } else {
    flag = false;
    -1 * s[1].to_string().parse::<isize>().unwrap()
  };
  
  let tv = if t[1] == 'F' {
    flag = flag;
    t[0].to_string().parse::<isize>().unwrap()
  } else {
    flag = !flag;
    -1 * t[1].to_string().parse::<isize>().unwrap()
  };
  
  let add = if flag {
    0
  } else {
    -1
  };
  
  println!("{}", (sv - tv).abs() + add);
}
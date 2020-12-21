#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};

const MOD:usize = 1_000_000_007;

fn convert(i:u8) -> String {
  (((i as u8) + ('a' as u8)) as char).to_string()
}

fn main() {
  input!{
    n: usize,
    rows: [Chars;n]
  }

  let mut result = vec![String::from("");n];
  let limit = n / 2;
  let add = n % 2;
  
  for i in 0..limit {
    let target = rows[n-1-i]
      .clone()
      .into_iter()
      .map(|v| v.to_string())
      .collect::<String>();
    let mut success = false;
    for ii in 0..n {
      let v = (rows[i][ii]).to_string();
      if target.find(&v).is_some() {
        result[i] = v.clone();
        result[n-1-i] = v;
        success = true;
      }
    }
    
    if !success {
      println!("-1");
      return
    }
  }

  if n % 2 == 1 {
    result[n/2] = rows[n/2][0].to_string();     
  }
  
  println!("{}", result.into_iter().collect::<String>());
} 
 
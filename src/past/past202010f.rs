#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use permutohedron::{Heap, heap_recursive};

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    mut k: usize,
    vals: [String;n]
  }
  let mut map = HashMap::new();
  for v in vals {
    let entry = map.entry(v).or_insert(0);
    *entry += 1;
  }
  
  let mut result = HashMap::new();
  for (key, v) in map {
    let mut entry = result.entry(v).or_insert(vec![]);
    entry.push(key);
  }
  
  let mut keys = result.keys().cloned().into_iter().collect::<Vec<usize>>();
  keys.sort();
  keys.reverse();
  
  for key in keys {
    let stack = result.get(&key).unwrap();
    let len = stack.len();
    if k < len {
      println!("AMBIGUOUS");
      return
    } else if len == 1 && k == 1 {
      println!("{}", stack[0]);
      return    
    } else if len == k {
      println!("AMBIGUOUS");
      return
    }
    k -= len;
  }
  
  println!("AMBIGUOUS");
}
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

fn main() {
  input!{
    n: usize,
    vals: [Chars;n]
  }
  
  let mut map = HashMap::new();
  map.insert('z', 0);
  map.insert('r', 0);
  map.insert('Z', 0);
  map.insert('R', 0);
  map.insert('b', 1);
  map.insert('c', 1);
  map.insert('B', 1);
  map.insert('C', 1);
  map.insert('d', 2);
  map.insert('w', 2);
  map.insert('D', 2);
  map.insert('W', 2);
  map.insert('t', 3);
  map.insert('j', 3);
  map.insert('T', 3);
  map.insert('J', 3);
  map.insert('f', 4);
  map.insert('q', 4);
  map.insert('F', 4);
  map.insert('Q', 4);
  map.insert('l', 5);
  map.insert('v', 5);
  map.insert('L', 5);
  map.insert('V', 5);
  map.insert('s', 6);
  map.insert('x', 6);
  map.insert('S', 6);
  map.insert('X', 6);
  map.insert('p', 7);
  map.insert('m', 7);
  map.insert('P', 7);
  map.insert('M', 7);
  map.insert('h', 8);
  map.insert('k', 8);
  map.insert('H', 8);
  map.insert('K', 8);
  map.insert('n', 9);
  map.insert('g', 9);
  map.insert('N', 9);
  map.insert('G', 9);
  
  let mut result = vec![];
  for s in vals {
    let mut temp = vec![];
    for c in s {
      if let Some(v) = map.get(&c) {
        temp.push(v.to_string());
      }
    }
    if !temp.is_empty() {
      result.push(temp.into_iter().collect::<String>());
    }
  }
  
  println!("{}", result.join(" "));
}
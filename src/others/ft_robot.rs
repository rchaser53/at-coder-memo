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
    s: Chars,
    x: isize,
    y: isize,
  }
  
  let mut xs = vec![0];
  let mut ys = vec![];
  let mut flag = true;
  
  for c in s {
    if c == 'F' {
      if flag {
        let li = xs.len()-1;
        xs[li] += 1;
      } else {
        let li = ys.len()-1;
        ys[li] += 1;
      }
    } else {
      if flag {
        ys.push(0);
      } else {
        xs.push(0);
      }
      flag = !flag;
    }
  }
  
  let mut dp = hashset![xs.remove(0)];
  for v in xs {
    let mut temp = hashset![];
    for vv in dp {
      temp.insert(vv+v);
      temp.insert(vv-v);
    }
    dp = temp;
  }
  if !dp.contains(&x) {
    println!("No");
    return
  }
  
  let mut dp = hashset![0];
  for v in ys {
    let mut temp = hashset![];
    for vv in dp {
      temp.insert(vv+v);
      temp.insert(vv-v);
    }
    dp = temp;
  }
  if !dp.contains(&y) {
    println!("No");
    return
  }
  
  println!("Yes");
}
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
    t: usize,
    vals: [(isize, isize, isize, isize);t]
  }
  
  for (x, y, p, q) in vals {
    let mut result = vec![];
    let n = 2 * (x + y);
    for l in x..x+y {
      match inverse(p+q,(l-p).rem_euclid(n),n) {
        None => continue,
        Some(v) => result.push(v*(p+q)+p),
      }
    }
    let n = p+q;
    for l in p..p+q {
      match inverse(2*(x+y),(l-x).rem_euclid(n),n) {
        None => continue,
        Some(v) => result.push(2*v*(x+y)+x),
      }
    }
    match result.into_iter().min() {
      None => println!("infinity"),
      Some(v) => println!("{}", v)
    }
  }  
}

// ax = b (mod n) となる最小のx
fn inverse(a:isize, b:isize, n:isize) -> Option<isize> {
  let (d, mut x, _y) = euclid(a, n);
  x = x.rem_euclid(n);
  if b % d != 0 {
    None
  } else {
    x = (x*(b/d)).rem_euclid(n);
    Some(x%(n/d))
  }
}

// ax+by = d
fn euclid(a:isize, b:isize) -> (isize, isize, isize) {
  funn(a,b,1,0,0,1)
}

fn funn(
  a:isize,
  b:isize,
  x1:isize,
  x2:isize,
  y1:isize,
  y2:isize
) -> (isize, isize, isize) {
  assert!(0<=b);
  if b == 0 {
    (a,x1,y1)
  } else {
    let r = a % b;
    let q = a / b;
    funn(b,r,x2,x1-q*x2,y2,y1-q*y2)
  }
}


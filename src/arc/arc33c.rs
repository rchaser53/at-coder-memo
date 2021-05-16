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

pub struct Fenwick<T> {
  zero: T,
  a: Box<[T]>,
}

impl<T> Fenwick<T>
where T: Copy + std::ops::Add<Output=T> {
  pub fn new(size:usize, zero:T) -> Fenwick<T> {
    Fenwick {
      zero,
      a: vec![zero;size+1].into_boxed_slice(),
    }
  }
  
  pub fn add(&mut self, mut x:usize, v:T) {
    assert!(x > 0);
    while let Some(a) = self.a.get_mut(x) {
      *a = *a + v;
      x += x & (!x + 1);
    }
  }
  pub fn sum(&self, mut x:usize) -> T {
    assert!(x < self.a.len());
    let mut res = self.zero;
    while 0 < x {
      res = res + self.a[x];
      x -= x & (!x + 1);
    }
    res
  }
}

impl<T> Fenwick<T>
where T:Copy + std::ops::Add<Output=T> + PartialOrd {
  pub fn search(&self, s:T) -> usize {
    let mut k = 1;
    while 2 * k < self.a.len() {
      k *= 2;
    }
    let mut x = 0;
    let mut w = self.zero;
    while 0 < k {
      self.a.get(x+k).map(|a| {
        if w + *a < s {
          w = w + *a;
          x += k;
        }  
      });
      k >>= 1;
    }
    x + 1
  }
}

fn main() {
  input!{
    q: usize,
    queries: [(usize, usize);q]
  }
  
  let mut bit = Fenwick::new(200_000,0);
  for (t, v) in queries {
    if t == 1 {
      bit.add(v, 1);
    } else {
      let v = bit.search(v);
      println!("{}", v);
      bit.add(v, !0);
    }
  }
}
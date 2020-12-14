#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use fenwicktree::FenwickTree;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    q: usize,
    vals: [usize;n],
    queries: [(usize, usize, usize);q]
  }
  
  let mut tree = FenwickTree::<usize>::new(n, 0);
  for i in 0..n {
    tree.add(i, vals[i]);
  }
  
  for (t, l, r) in queries {
    if t == 1 {
      tree.add(l-1, r);
    } else {
      println!("{}", tree.sum(l-1, r));
    }
  }
}

pub mod fenwicktree {
  // Reference: https://en.wikipedia.org/wiki/Fenwick_tree
  pub struct FenwickTree<T> {
    n: usize,
    ary: Vec<T>,
    e: T,
  }

  impl<T: Clone + std::ops::AddAssign<T> + std::ops::BitXorAssign<T>> FenwickTree<T> {
    pub fn new(n: usize, e: T) -> Self {
      FenwickTree {
        n,
        ary: vec![e.clone(); n],
        e,
      }
    }
    pub fn accum(&self, mut idx: usize) -> T {
      let mut sum = self.e.clone();
      while idx > 0 {
        sum ^= self.ary[idx - 1].clone();
        idx &= idx - 1;
      }
      sum
    }
    /// performs data[idx] ^= val;
    pub fn add<U: Clone>(&mut self, mut idx: usize, val: U)
      where
        T: std::ops::BitXorAssign<U>,
    {
      let n = self.n;
      idx += 1;
      while idx <= n {
        self.ary[idx - 1] ^= val.clone();
        idx += idx & idx.wrapping_neg();
      }
    }
    /// Returns data[l] ^ ... ^ data[r - 1].
    pub fn sum(&self, l: usize, r: usize) -> T
      where
        T: std::ops::BitXor<Output = T>,
    {
      self.accum(r) ^ self.accum(l)
    }
  }
}
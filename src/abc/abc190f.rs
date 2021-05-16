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
    vals: [usize;n]
  }
  
  let mut count = 0;
  let mut memo = vec![0;n+1];
  for i in 0..n {
    add(&mut memo, vals[i] + 1, 1);
    count += i + 1 - sum(&memo, vals[i] + 1);
  }
  
  println!("{}", count);
  for i in 0..n-1 {
    let v = vals[i];
    count += n - 2 * v - 1;
    println!("{}", count);
  }
}

fn add(bit:&mut Vec<usize>, i:usize, x:usize) {
  let mut j = i;
  while j < bit.len() {
    bit[j] += x;
    j += lsb(j);
  }
}

fn sum(bit: &Vec<usize>, i:usize) -> usize {
  if i == 0 {
    0
  } else {
    bit[i] + sum(bit, i - lsb(i))
  }
}

fn lsb(i: usize) -> usize {
  ((i as isize) & -(i as isize)) as usize
}

/* 以下セグメントツリーでの解法 */
pub fn main(
) {
    input! {
      n:usize,
      vals:[usize;n]
    }
    
    let mut stree = SegmentTree::new(n+1, 0);
    let mut count = 0;
    for i in 0..n {
      stree.update(vals[i]+1, 1);
      count += i - stree.query(0, vals[i]+1);
    }
    println!("{}", count);
    for i in 0..n-1 {
      count += n - 1;
      count -= vals[i] * 2;
      println!("{}", count);
    }
}

type TreeType = usize;
pub struct SegmentTree {
  n: usize,
  arr: Vec<TreeType>,
  e: TreeType
}
impl SegmentTree {
  pub fn new(n: usize, e:TreeType) -> Self {
    let mut x = 1;
    while x < n {
      x *= 2;
    }
    SegmentTree {
      n:x,
      arr:vec![e;x*2],
      e
    }    
  }
   
  pub fn update(&mut self, mut i: usize, val: TreeType) {
    i += self.n - 1;
    self.arr[i] = val;
    while 0 < i {
      i = (i-1) / 2;
      self.arr[i] = self.arr[i*2+1] + self.arr[i*2+2];
    }
  }
  
  // a以上、b未満になるのに注意
  fn query(&self, a:usize, b:usize) -> TreeType {
    self.query_sub(a, b, 0, 0, self.n)
  }

  fn query_sub(
    &self, a:usize, b:usize,
    k:usize, l:usize, r:usize
  ) -> TreeType { 
    if r <= a || b <= l {
      0
    } else if a <= l && r <= b {
      self.arr[k]
    } else {
      let vl = self.query_sub(a, b, k*2+1, l, (l+r)/2);
      let vr = self.query_sub(a, b, k*2+2, (l+r)/2, r);
      vl + vr
    }
  }
}

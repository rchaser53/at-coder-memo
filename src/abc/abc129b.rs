#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use petgraph::unionfind::UnionFind;

const MOD:usize = 1_000_000_007;
#[fastout]
fn main() {
  input!{
    n:usize,
    vals:[isize;n]
  }
  let mut memo = vec![0;n+1];
  for i in 0..n {
    memo[i+1] = vals[i] + memo[i];
  }
  
  let mut min = 1_000_000_000;
  for i in 1..n {
    min = std::cmp::min(min, (memo[n] - memo[i] - memo[i]).abs());
  }
  println!("{}", min);
}

#[fastout]
fn main() {
  input!{
    n:usize,
    vals:[isize;n]
  }
  let mut memo = vec![0;n+1];
  for i in 0..n {
    memo[i+1] = vals[i] + memo[i];
  }
  let mut stree = SegmentTree::new(n, 0);
  for i in 0..n {
    stree.update(i, vals[i]);
  }
  
  let mut min = 1_000_000_000;
  for i in 0..n-1 {
    let v = (stree.query(0, i+1) - stree.query(i+1, n)).abs();
    min = std::cmp::min(min, v);
  }
  println!("{}", min);
}

type TreeType = isize;
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

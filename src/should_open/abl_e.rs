/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::Ordering;

const MOD:usize = 998_244_353;
const MAX_VALUE:usize = std::usize::MAX;
// 区間和を求める遅延セグ木
struct LazySegmentTree {
  node: Vec<usize>,
  nums: Vec<usize>,
  lazy: Vec<usize>,
  pub n: usize
}

impl LazySegmentTree {
  pub fn new(nums:Vec<usize>) -> Self {
    let size = nums.len();
 
    let mut n = 1;
    while n < size {
        n *= 2;
    }
    let mut node = vec![0; 2 * n - 1];

    for i in 0..size {
      node[i+n-1] = nums[i];
    }

    if n > 2 {
      for i in (0..=(n-2)).rev() {
        node[i] = (node[2*i+1] + node[2*i+2]) % MOD;
      }
    }

    Self {
      node: node.clone(),
      nums: node,
      n,
      lazy: vec![MAX_VALUE;2*n-1],
    }
  }

  fn eval(&mut self, k:usize) {
    if self.lazy[k] == MAX_VALUE {
      return
    }
    if k < self.n - 1 {
      self.lazy[k*2+1] = self.lazy[k];
      self.lazy[k*2+2] = self.lazy[k];
    }

    self.node[k] = self.lazy[k] * self.nums[k] % MOD;
    self.lazy[k] = MAX_VALUE;
  }

  pub fn update_range(
    &mut self,
    a:usize,
    b:usize,
    x:usize,
    k:usize,
    l:usize,
    r:usize
  ) {
    self.eval(k);
    if a <= l && r <= b {
      self.lazy[k] = x;
      self.eval(k);
    } else if a < r && l < b {
      self.update_range(a, b, x, 2*k+1, l, (l+r)/2);
      self.update_range(a, b, x, 2*k+2, (l+r)/2, r);
      
      self.node[k] = (self.node[2*k+1] + self.node[2*k+2]) % MOD;
    }
  }

  pub fn update(&mut self, a:usize, b:usize, x:usize) {
    self.update_range(a, b, x, 0, 0, self.n);
  }

  pub fn query_range(
    &mut self,
    a:usize,
    b:usize,
    k:usize,
    l:usize,
    r:usize,
  ) -> usize {
    self.eval(k);
    if r <= a || b <= l {
      0
    } else if a <= l && r <= b {
      self.node[k]
    } else {
      let c1 = self.query_range(a, b, 2*k+1, l, (l+r)/2);
      let c2 = self.query_range(a, b, 2*k+2, (l+r)/2, r);
      (c1 + c2) % MOD
    }
  }

  pub fn query(&mut self, a:usize, b:usize) -> usize {
    self.query_range(a, b, 0, 0, self.n)
  }
}

fn main() {
  input!{
    n:usize,
    q:usize,
    vals:[(usize,usize,usize);q]
  }

  let mut nums = vec![0;n];
  nums[0] = 1;
  for i in 1..nums.len() {
    nums[i] = nums[i-1] * 10 % MOD;
  }

  let mut segtree = LazySegmentTree::new(nums);
  for (a, b, d) in vals {
    segtree.update(n-b, n-a+1, d);
    println!("{}", segtree.query(0, n));
  }
}
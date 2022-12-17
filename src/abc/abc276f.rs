/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

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

const MOD:usize = 998244353;
fn mod_pow(a: usize, n: usize) -> usize { 
  if n == 0 {
    1
  } else if n == 1 {
    a % MOD
  } else if n % 2 == 1 {
    let k = mod_pow(a, (n - 1) / 2);
    let ans = (a * k) % MOD;
    (ans * k) % MOD
  } else {
    let k = mod_pow(a, n / 2);
    (k * k) % MOD
  }
}

// a^{-1} mod を計算する
fn mod_inv(a:usize) -> usize {
  mod_pow(a, MOD-2)
}

fn main() {
  input! {
    n:usize,
    a:[usize;n]
  }

  let limit = 2*10usize.pow(5)+10;
  let mut num_seg = SegmentTree::new(limit+10, 0);
  let mut tot_seg = SegmentTree::new(limit+10, 0);
  
  let mut numerator = 0;
  for i in 0..n {
    let v = a[i];
    let denominator = (i+1) * (i+1) % MOD;

    numerator += a[i];
    numerator %= MOD;

    let below = num_seg.query(0, v+1) * a[i];
    let up = tot_seg.query(v+1, limit);
    let add = (below + up) % MOD * 2 % MOD;
    numerator += add;
    numerator %= MOD;

    println!("{}", numerator * mod_inv(denominator) % MOD);

    let cv = num_seg.query(v, v+1);
    num_seg.update(v, cv+1);

    let cv = tot_seg.query(v, v+1);
    tot_seg.update(v, (cv+v) % MOD);
  }
}
/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
    input! {
      w:usize,
      n:usize,
      vals:[(isize,isize,isize);n]
    }
    let mut last = SegmentTree::new(w+10, -1);
    last.update(0, 0);
    
    for i in 0..n {
      let mut next = last.clone();
      let (l, r, v) = vals[i];
      let lu = l as usize;
      for j in lu..=w {
        let int_j = j as isize;
        let left = std::cmp::max(0, int_j-r) as usize;
        let right = std::cmp::max(0, int_j-l) as usize;
        let temp_max = last.query(left, right+1);
        if temp_max == -1 { continue }
        let cv = last.query(j, j+1);
        if cv < temp_max+v {
          next.update(j, temp_max+v);
        }
      }
      last = next.clone();
    }
    println!("{}", last.query(w, w+1));
}

type TreeType = isize;
#[derive(Clone)]
pub struct SegmentTree {
  n: usize,
  arr: Vec<TreeType>,
  pub e: TreeType
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
    i += self.n-1;
    self.arr[i] = val;
    while 0 < i {
      i = (i-1) / 2;
      self.arr[i] = std::cmp::max(self.arr[i*2+1], self.arr[i*2+2]);
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
      self.e
    } else if a <= l && r <= b {
      self.arr[k]
    } else {
      let vl = self.query_sub(a, b, k*2+1, l, (l+r)/2);
      let vr = self.query_sub(a, b, k*2+2, (l+r)/2, r);
      std::cmp::max(vl,  vr)
    }
  }
}

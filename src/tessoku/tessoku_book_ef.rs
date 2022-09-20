/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;

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

// 転倒数
// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ef
fn main() {
  input! {
    n:usize,
    a:[usize;n],
  }

  let mut result = 0usize;
  let mut stree = SegmentTree::new(n+10, 0);
  for i in 0..n {
    let v = a[i];
    stree.update(v, 1);

    let sum = stree.query(0, v); // vより小さい奴ら
    let nv = i - sum;            // 現在のindexから引いてやれば転倒数の数が分かる (indexは0開始で1少ないが自分は含めないので丁度良い)
    result += nv;
  }
  println!("{}", result);
}
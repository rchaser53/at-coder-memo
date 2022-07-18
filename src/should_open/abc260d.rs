/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
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
    i += self.n-1;
    self.arr[i] = val;
    while 0 < i {
      i = (i-1) / 2;
      self.arr[i] = std::cmp::min(self.arr[i*2+1], self.arr[i*2+2]);
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
      std::cmp::min(vl,  vr)
    }
  }
}

fn main() {
  input! {
    n:usize,
    k:usize,
    vals:[usize;n],
  }

  let inf = 1_000_000_000;
  let limit = n+10;
  let mut stree = SegmentTree::new(limit, inf);
  let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
  let mut result = vec![-1;n];
  for i in 0..n {
    let v = vals[i];
    let ti = stree.query(v+1, limit);
    let arr = if ti == inf {
      stree.update(v, v);
      vec![v-1]
    } else {
      let mut arr = map.remove(&ti).unwrap();
      stree.update(ti, inf);
      stree.update(v, v);
      arr.push(v-1);
      arr
    };
      
    if arr.len() == k {
      for j in arr {
        result[j] = (i + 1) as i32;
      }
      stree.update(v, inf);
    } else {
      map.insert(v, arr);
    }
  }

  for v in result {
    println!("{}", v);
  }
}
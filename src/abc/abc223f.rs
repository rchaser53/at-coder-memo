/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering::*;

type TreeType = (isize, isize);
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
      self.arr[i].0 = self.arr[i*2+1].0 + self.arr[i*2+2].0;
      self.arr[i].1 = std::cmp::min(self.arr[i*2+1].1, self.arr[i*2+1].0 + self.arr[i*2+2].1);
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
      (0, 0)
    } else if a <= l && r <= b {
      self.arr[k]
    } else {
      let vl = self.query_sub(a, b, k*2+1, l, (l+r)/2);
      let vr = self.query_sub(a, b, k*2+2, (l+r)/2, r);
      (vl.0 + vr.0, std::cmp::min(vl.1, vl.0 + vr.1))
    }
  }
}

fn helper(stree: &mut SegmentTree, c:char, i:usize) {
  if c == '(' {
    stree.update(i, (1,1));
  } else {
    stree.update(i, (-1,-1));
  }
}

fn main() {
  input! {
    n:usize,
    q:usize,
    mut s:Chars,
    queries:[(usize, Usize1, Usize1);q]
  }

  let mut stree = SegmentTree::new(n+10, (0,0));
  for i in 0..n {
    helper(&mut stree, s[i], i);
  }

  for (t, l, r) in queries {
    if t == 1 {
      s.swap(l, r);
      helper(&mut stree, s[l], l);
      helper(&mut stree, s[r], r);
    } else {
      let (v1, v2) = stree.query(l, r+1);
      if v1 == 0 && v2 >= 0 {
        println!("Yes");
      } else {
        println!("No");
      }
    }
  }
}
/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
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

fn compress<T: std::cmp::Eq + std::cmp::Ord + std::hash::Hash + Copy >(arr:&Vec<T>) -> (Vec<T>, HashMap<T, usize>) {
  let mut set = HashSet::new();
  for &v in arr {
    set.insert(v);
  }
  let mut arr = set.into_iter().collect::<Vec<T>>();
  arr.sort();

  let mut map = HashMap::new();
  for i in 0..arr.len() {
    map.insert(arr[i], i);
  }
  (arr, map)
}

fn main() {
  input! {
    n:usize,
    k:usize,
    vals:[usize;n]
  }

  let (arr, map) = compress(&vals);
  let inf = 1_000_000_000_000;
  let len = arr.len();
  let mut stree = SegmentTree::new(len+10, inf);
  let mut set = HashSet::new();
  for i in k..n {
    let v = vals[i];
    if !set.contains(&v) {
      set.insert(v);
      stree.update(*map.get(&vals[i]).unwrap(), i);
    }
  }

  let mut result = inf;
  for i in 0..k {
    let ti = stree.query(*map.get(&vals[i]).unwrap()+1, arr.len()+3);
    if ti != inf {
      result = std::cmp::min(result, ti - i);
    }
  }

  if result == inf {
    println!("-1");
  } else {
    println!("{}", result);
  }
}
/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering::*;

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

fn main() {
  input! {
    n:usize,
    a:[usize;n],
    b:[usize;n],
  }

  let mut ab = vec![(0,0);n];
  for i in 0..n {
    ab[i] = (a[i], b[i]);
  }

  ab.sort_by(|a,b| {
    let v = a.1.cmp(&b.1);
    if v == Equal {
      if a.0.cmp(&b.0) == Less {
        Greater
      } else {
        Less
      }
    } else {
      v
    }
  });

  let mut set = HashSet::new();
  for i in 0..n {
    set.insert(ab[i].0);
  }
  let mut arr = set.into_iter().enumerate().collect::<Vec<(usize, usize)>>();
  arr.sort_by(|a,b| a.1.cmp(&b.1));

  let len = arr.len();
  let mut map = HashMap::new();
  
  for i in 0..len {
    map.insert(arr[i].1, i);
  }
  

  let mut result = 0usize;
  let mut arr = vec![];

  for (v, _) in ab {
    if arr.is_empty() {
      arr.push((v,1));
    } else if arr[arr.len()-1].0 == v {
      let l = arr.len()-1;
      arr[l].1 += 1;
    } else {
      arr.push((v,1));
    }
  }

  let mut stree = SegmentTree::new(arr.len()+10, 0);
  let mut ci = 0;
  for (v, num) in arr {
    let rank = *map.get(&v).unwrap();
    // 現在の数を取得する
    let v_num = stree.query(rank, rank+1);
    stree.update(rank, v_num+num);

    ci += num;
    // 自分より小さい
    let sum = stree.query(0, rank);
    let nv = (ci - sum) * num;

    result += nv;
    // println!("original value:{} rank:{} sum:{} nv:{}", v, rank, sum, nv);
  }
  println!("{}", result);
}
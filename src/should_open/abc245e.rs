/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;
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


fn compress(arr:&Vec<(usize,usize,u8)>) -> HashMap<usize, usize> {
  let mut set = HashSet::new();
  for &v in arr {
    set.insert(v.1);
  }
  let mut arr = set.into_iter().collect::<Vec<usize>>();
  arr.sort();

  let mut map = HashMap::new();
  for i in 0..arr.len() {
    map.insert(arr[i], i);
  }
  map
}

fn main() {
  input! {
    n:usize,
    m:usize,
    a:[usize;n],
    b:[usize;n],
    c:[usize;m],
    d:[usize;m]
  }

  let mut arr = vec![(0,0,0);n+m];
  for i in 0..n {
    arr[i] = (a[i], b[i],1);
  }
  for i in 0..m {
    arr[n+i] = (c[i], d[i],0);
  }
  let dict = compress(&arr);
  let k = dict.keys().len();

  let inf = 1_000_000_000;
  let limit = k+10;
  let mut segtree = SegmentTree::new(limit, inf);

  arr.sort_by(|a,b| {
    let v = a.0.cmp(&b.0);
    if v == Ordering::Equal {
      a.2.cmp(&b.2)
    } else if v == Ordering::Less {
      Ordering::Greater
    } else {
      Ordering::Less
    }
  });

  let mut map = HashMap::new();
  for (_, c, t) in arr {
    let v = *dict.get(&c).unwrap();

    if t == 0 {
      *map.entry(v).or_insert(0) += 1;
      segtree.update(v, v);
    } else {
      let ti = segtree.query(v, k+1);
      if ti == inf {
        println!("No");
        return
      }

      let entry = map.entry(ti).or_insert(0);
      if *entry == 1 {
        map.remove(&ti);
        segtree.update(ti, inf);
      } else {
        *entry -= 1;
      }
    }
  }

  println!("Yes");
}
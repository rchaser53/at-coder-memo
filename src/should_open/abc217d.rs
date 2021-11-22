use proconio::input;
use proconio::marker::*;
use std::collections::*;

type Target = usize;
type UseValue = usize;

// 二分探索 + O(logN) orderでinsert
fn lower_bound(stree: &SegmentTree, x: &UseValue) -> usize {
    let limit = stree.n+1;
    let mut low = 0;
    let mut high = limit;
    while low != high {
        let mid = (low + high) / 2;
        // NEEDS TO EDIT
        let v = stree.query(0, mid);
        match v.cmp(x) {
            std::cmp::Ordering::Less => {
                low = mid + 1;
            }
            std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                high = mid;
            }
        }
    }
    low
}

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
      0
    } else if a <= l && r <= b {
      self.arr[k]
    } else {
      let vl = self.query_sub(a, b, k*2+1, l, (l+r)/2);
      let vr = self.query_sub(a, b, k*2+2, (l+r)/2, r);
      std::cmp::max(vl,  vr)
    }
  }
}

fn main() {
  input!{
    l:usize,
    q:usize,
    vals:[(usize, usize);q]
  }

  let mut set = HashSet::new();
  set.insert(0);
  for &(_, v) in &vals {
    set.insert(v);
  }
  let mut dict = set.into_iter().collect::<Vec<usize>>();
  dict.sort();

  let mut map = HashMap::new();
  map.insert(0, 0);
  for i in 1..dict.len() {
    map.insert(dict[i], i);
  }

  let qn = dict.len();
  let mut stree = SegmentTree::new(qn+1, 0);
  for (t, v) in vals {
    let ti = *map.get(&v).unwrap();
    if t == 1 {
      stree.update(ti, ti);
    } else {
      let li = stree.query(0, ti);
      let lv  = dict[li];

      let hi = lower_bound(&stree, &ti) - 1;
      let hv = if qn <= hi {
        l
      } else {
        dict[hi]
      };

      println!("{}", hv - lv);
    }
  }
}
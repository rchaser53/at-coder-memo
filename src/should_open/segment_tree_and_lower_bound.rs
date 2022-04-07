/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

// セグ木とlower_bound
type TreeType = usize;
fn lower_bound(stree: &SegmentTree, x:TreeType) -> usize {
    let mut low = 0;
    let mut high = stree.arr.len();
    while low != high {
        let mid = (low + high) / 2;

        // 0からmidまでのindexの間の最大値を返す
        let v = stree.query(0, mid+1);
        // NEEDS TO EDIT
        match v.cmp(&x) {
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

pub fn sol(rains: Vec<i32>) -> Vec<i32> {
  let n = rains.len();
  let mut stree = SegmentTree::new(n+10, 0);
  let mut set = HashSet::new();
  let mut result = vec![-1;n];
  
  let mut map = HashMap::new();
  for si in 1..=n {
    let i = si - 1;
    let v = rains[i];

    if let Some(&ni) = map.get(&v) {
      // dryできる最小の値
      let ti = lower_bound(&stree, ni);
      if si <= ti {
        return vec![]
      }

      stree.update(ti, 0);
      set.remove(&ti);
      map.insert(v, si);
      result[ti - 1] = v as i32;
    } else if v == 0 {
      stree.update(si, si);
      set.insert(si);
    } else {
      map.insert(v, si);
    }
  }

  for ti in set {
    result[ti-1] = 1;
  }

  result
}

fn main() {
  input! {
    n: usize,
    a: [i32;n]
  }
  println!("{}", sol(a).into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}
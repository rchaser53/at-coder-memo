/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

type Target = usize;
type UseValue = usize;
fn lower_bound(stree: &SegmentTree, x:isize) -> usize {
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

type TreeType = isize;
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

// 座標圧縮
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
    q:usize,
  }

  let mut queries = vec![];
  let mut vals = vec![0];
  for _ in 0..q {
    input!{
      t:usize,
      x:usize
    }
    vals.push(x);
    let k = if t == 1 {
      1
    } else {
      input! {
        k:usize
      }
      k
    };
    queries.push((t,x,k));
  }

  let (origins, map) = compress(&vals);
  let n = origins.len();

  let mut stree = SegmentTree::new(n+10, -1);
  let mut counts = HashMap::new();

  for (t, x, k) in queries {
    let mut compressed = *map.get(&x).unwrap();
    if t == 1 {
      *counts.entry(compressed).or_insert(0) += 1;
      stree.update(compressed, compressed as isize);
      continue
    }

    let mut tot = 0;
    if t == 2 {
      loop {
        // x以下の最大の値
        let temp  = stree.query(0, compressed+1);
        if temp == -1 {
          println!("-1");
          break
        }
        compressed = temp as usize;
        tot += *counts.get(&compressed).unwrap_or(&0);

        if k <= tot {
          println!("{}", origins[compressed]);
          break
        } else if compressed == 0 {
          println!("-1");
          break
        }
        compressed -= 1;
      }
    } else {
      let mut last = 0;
      loop {
        // x以上の最小の値
        compressed = lower_bound(&stree, compressed as isize);
        tot += *counts.get(&compressed).unwrap_or(&0);

        if compressed == last || n <= compressed {
          println!("-1");
          break
        } else if k <= tot {
          println!("{}", origins[compressed]);
          break
        }
        last = compressed;
        compressed += 1;
      }
    }
  }
}
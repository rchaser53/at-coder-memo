/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering::*;

type Target = usize;
type UseValue = usize;
fn lower_bound(stree: &SegmentTree, x: &UseValue) -> usize {
    let mut low = 0;
    let mut high = stree.arr.len();
    while low != high {
        let mid = (low + high) / 2;

        // 0からmidまでのindexの間の最大値を返す
        let v = stree.query(0, mid+1);
        // NEEDS TO EDIT
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

fn main() {
  input! {
    l:usize,
    q:usize,
    queries:[(usize,usize);q]
  }

  // 座標圧縮
  let mut set = HashSet::new();
  for &(_, v) in &queries {
    set.insert(v);
  }
  set.insert(0);
  set.insert(l);

  let mut arr = set.into_iter().collect::<Vec<usize>>();
  arr.sort();
  let mut map = HashMap::new();
  for i in 0..arr.len() {
    map.insert(arr[i], i);
  }
  // 座標圧縮

  let len = arr.len();
  let mut stree = SegmentTree::new(arr.len()+10, 0);
  for (t, v) in queries {
    let ti = *map.get(&v).unwrap();
    if t == 1 {
      stree.update(ti, ti);
    } else {
      // tiより小さい中で最大の値が取得できる
      let li = stree.query(0, ti);
      // 複数同じ要素がある訳でないのでlower_boundでなくても良い(エラーハンドリングが楽なだけ)
      // tiより大きい中で最低の値を取得したい
      let hi = lower_bound(&stree, &(ti+1));

      // 範囲外の場合はlenより大きい値になるためエラーハンドリング
      if len < hi {
        println!("{}", l - arr[li]);
      } else {
        println!("{}", arr[hi] - arr[li]);
      }
    }
  }
}
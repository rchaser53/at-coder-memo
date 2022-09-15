/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;

// 平面走査
// 連続部分配列のユニークな値の数を数える際は同じ値が出たら-1するという発想をする。
// どうやって数えるかというと前の同じ値のindexを持つ。これを変形して

// (x, y) = (前の同じ値のindex, index)と考える
// (a,b)の範囲の連続部分配列のユニークな数を求めたい時
// これは a <= x && y <= bに含まれる点の数が重複した数

// これは平面走査で求められる。平面捜査にはsumが使えるセグ木を使う
// xを大きい値からずらしていく。セグ木を使ってyの値のindexに+1する

// すると(a,b)の範囲のユニークな数がsumができるセグ木でO(logN)で求められる
// ここで求められるのは重複した値の数になる
// (b - a + 1) - segtree.query(b, a+1) みたいな感じ
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
    q:usize,
    vals:[usize;n],
    mut queries:[(usize,usize);q]
  }

  let queries = queries.into_iter().enumerate().map(|(i,v)| 
    (v.0, v.1, i)
  ).collect::<Vec<(usize,usize,usize)>>();
  let mut map = HashMap::new();
  for i in 0..n {
    map.entry(vals[i]).or_insert(vec![]).push(i);
  }

  let inf = 1_000_000_000;
  let mut bars = vec![];
  for (_, arr) in map {
    let mut ri = inf-1;
    for &li in arr.iter().rev() {
      bars.push((li+1, ri+1, 0, 1));
      ri = li;
    }
  }

  for (l,r,i) in queries {
    bars.push((l,r,i,0));
  }

  bars.sort_by(|a,b| {
    let v = a.0.cmp(&b.0);
    if v == Ordering::Equal {
      a.3.cmp(&b.3)
    } else {
      v
    }
  });
  bars.reverse();

  let len = 5*10usize.pow(5)+10;
  let mut stree = SegmentTree::new(len, 0);
  let mut result = vec![0;q];
  for (l,r,i,t) in bars { 
    // vals
    if t == 1 {
      if r == inf { continue }
      let cv = stree.query(r,r+1);
      stree.update(r, cv+1);
    }
    // queries
    else {
      let v = stree.query(l, r+1);
      result[i] = r - l + 1 - v;
    }
  }

  for v in result {
    println!("{}", v);
  }
}
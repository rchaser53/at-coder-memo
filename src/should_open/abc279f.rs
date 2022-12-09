/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

// usizeでsizeも使える union_find
struct UnionFind {
  parents: Vec<usize>,
  sizes: Vec<usize>,
  ranks: Vec<usize>
}

impl UnionFind {
  fn new(n:usize) -> UnionFind {
    let mut uf = UnionFind { parents: vec![0;n], sizes:vec![1;n], ranks:vec![0;n] };
    for i in 0..n {
      uf.parents[i] = i;
    }
    uf
  }

  fn root(&mut self, x:usize) -> usize {
    let ti = self.parents[x];
    if ti == x {
      x
    } else {
      self.parents[x] = self.root(ti);
      self.parents[x]
    }
  }

  fn is_same(&mut self, x:usize, y:usize) -> bool {
    self.root(x) == self.root(y)
  }

  fn unite(&mut self, x:usize, y:usize) -> bool {
    let rx = self.root(x);
    let ry = self.root(y);
    if rx == ry { return false }
    let (rx, ry) = if self.ranks[rx] < self.ranks[ry] {
      (ry, rx)
    } else {
      (rx, ry)
    };
    self.parents[ry] = rx;
    if self.ranks[rx] == self.ranks[ry] {
      self.ranks[rx] += 1;
    }

    self.sizes[rx] += self.sizes[ry];
    true
  }

  fn group_size(&mut self, x:usize) -> usize {
    let i = self.root(x);
    self.sizes[i]
  }
}


fn main() {
  input! {
    n:usize,
    q:usize,
  }

  let limit = 1_000_000;
  let mut uf = UnionFind::new(limit);
  let mut count = n;
  
  // union_findと逆引きを利用することで集合のマージを高速にできる
  let mut box2g = vec![0;limit+1];
  let mut g2box = vec![0;limit+1];
  for i in 0..=limit {
    box2g[i] = i;
    g2box[i] = i;
  }

  for _ in 0..q {
    input! {
      t: usize
    }

    if t == 1 {
      input! {
        x:usize,
        y:usize,
      }

      if box2g[y] == 0 {
        continue
      }

      let gy = box2g[y];
      if box2g[x] == 0 {
        box2g[y] = 0;
        box2g[x] = gy;
        g2box[uf.root(gy)] = x;
        continue
      }
    
      let gx = box2g[x];
      box2g[x] = 0;
      box2g[y] = 0;
      uf.unite(gx, gy);
      
      let rx = uf.root(gx);
      box2g[x] = rx;
      g2box[rx] = x;
    } else if t == 2 {
      input! {
        x:usize,
      }

      count += 1;
      box2g[count] = 0;

      if box2g[x] == 0 {
        box2g[x] = count;
        g2box[count] = x;
      } else {
        let gx = box2g[x];
        box2g[x] = 0;

        uf.unite(gx, count);
        let rx = uf.root(gx);
        box2g[x] = rx;
        g2box[rx] = x;
      }

    }
    // t == 3
    else {
      input! {
        x:usize,
      }
      println!("{}", g2box[uf.root(x)]);
    }
  }
}
/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

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
  input!{
    n:usize,
    m:usize,
    uv:[(Usize1,Usize1);m],
    k:usize,
    xy: [(Usize1,Usize1);k],
    q:usize,
    pq:[(Usize1,Usize1);q]
  }

  let mut tree = UnionFind::new(n);

  for (u, v) in uv {
    tree.unite(u,v);
  }

  let mut set = HashSet::new();
  for (x, y) in xy {
    let mut xpi = tree.root(x);
    let mut ypi = tree.root(y);

    if ypi < xpi {
      std::mem::swap(&mut xpi, &mut ypi);
    }
    set.insert((xpi, ypi));
  }

  for (x, y) in pq {
    let mut xpi = tree.root(x);
    let mut ypi = tree.root(y);

    if ypi < xpi {
      std::mem::swap(&mut xpi, &mut ypi);
    }

    if set.contains(&(xpi,ypi)) {
      println!("No");
    } else {
      println!("Yes");
    }
  }
  
}
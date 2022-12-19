/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn repeat_square(n:usize, p:usize, m:usize) -> usize {
  if p == 0 {
    1
  } else if p % 2 == 0 {
    repeat_square(n, p/2, m).pow(2) % m 
  } else {
    n * repeat_square(n, p-1, m) % m
  }
}

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

fn clascal(
  edges: &Vec<(usize,usize,usize)>,
  n:usize
) -> usize {
  let mut sum = 0usize;
  let mut uf = UnionFind::new(n);
  for &(v,a,b) in edges {
    if !uf.is_same(a,b) {
      sum += v;
      uf.unite(a,b);
    }
  }
  sum
}

fn main() {
  input! {
    n:usize,
    m:usize,
    a:[usize;n]
  }

  let mut edges = vec![];
  for i in 0..n {
    for j in i+1..n {
      let v = (repeat_square(a[i], a[j], m) + repeat_square(a[j], a[i], m)) % m;
      edges.push((v, i, j));
      edges.push((v, j, i));
    }
  }

  edges.sort();
  edges.reverse();
  println!("{}", clascal(&edges, n));
}
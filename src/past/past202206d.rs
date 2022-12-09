/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
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

fn helper(a:char) -> usize {
  (a as u8 - 'a' as u8) as usize
}

fn main() {
  input! {
    n:usize,
    ab:[(char,char);n],
    s:Chars,
    t:Chars,
  }

  let mut uf = UnionFind::new(26);
  for (a, b) in ab {
    uf.unite(helper(a), helper(b));
  }

  for i in 0..s.len() {
    if !uf.is_same(helper(s[i]),  helper(t[i])) {
      println!("No");
      return
    }
  }
  println!("Yes");
}
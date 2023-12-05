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

struct Helper {
  n:usize,
  k:usize,
  result: usize,
  memo: Vec<Vec<usize>>,
}

impl Helper {
  fn dfs(&mut self, ci:usize, val:usize, p: &mut Vec<usize>) {
    if ci == self.n-1 {
      
      let mut tree = UnionFind::new(self.n);
      for i in 0..self.n-1 {
        if p[i] != 10 {
          tree.unite(i, p[i]);
        }
      }

      if tree.group_size(0) == self.n {
        // println!("{:?} {}", &p, val);
        self.result = self.result.min(val % self.k);
      }

      return 
    }

    for ni in 0..self.n {
      if ni == ci || self.memo[ci][ni] == 10usize.pow(17) { continue }
      p[ci] = ni;
      self.dfs(ci+1, val + self.memo[ci][ni], p);
    }
  }
}

fn main() {
  input! {
    n:usize,
    m:usize,
    k:usize,
    mut uvw:[(Usize1,Usize1,usize);m]
  }

  let inf = 10usize.pow(17);
  let mut memo = vec![vec![inf;n];n];

  for (a,b,v) in uvw {
    memo[a][b] = memo[a][b].min(v);
    memo[b][a] = memo[b][a].min(v);
  }

  let mut helper = Helper { memo, n, result: inf, k };
  let mut p = vec![10;n];
  helper.dfs(0,0, &mut p);

  println!("{}", helper.result % k);
}
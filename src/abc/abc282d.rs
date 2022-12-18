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

fn helper(a:usize) -> usize {
  if a == 0 {
    1
  } else {
    0
  }
}

fn main() {
  input! {
    n:usize,
    m:usize,
    edges:[(Usize1,Usize1);m]
  }

  let mut g = vec![vec![];n];
  let mut uf = UnionFind::new(n);
  let mut set = HashSet::new();
  for (a, b) in edges {
    g[a].push(b);
    g[b].push(a);
    uf.unite(a, b);

    if a < b {
      set.insert((a,b));
    } else {
      set.insert((b,a));
    }
  }

  let default = 100;

  let mut group = vec![default;n];
  for i in 0..n {
    if group[i] != default { continue }
    group[i] = 0;
    let mut stack = vec![(i,0)];
    while !stack.is_empty() {
      let mut new_stack = vec![];
      while let Some((ci, t)) = stack.pop() {
        let next_type = helper(t);
        for &ni in &g[ci] {
          if group[ni] == default {
            group[ni] = next_type;
            new_stack.push((ni, next_type));
          } else if group[ni] != next_type {
            println!("0");
            return
          }
        }
      }
      stack = new_stack;
    }
  }

  let mut lands = HashMap::new();
  for i in 0..n {
    let pi = uf.root(i);
    lands.entry(pi).or_insert(vec![]).push(i);
  }

  let mut result = n*(n-1)/2-m;  
  for (_, arr) in lands {  
    let mut black = 0;
    let mut white = 0;
    for i in arr {
      if group[i] == 1 {
        black += 1;
      } else {
        white += 1;
      }
    }

    result -= black * (black-1) / 2;
    result -= white * (white-1) / 2;
  }
  println!("{}", result);
}
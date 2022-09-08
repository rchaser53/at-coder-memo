/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;

struct LCA {
  depths: Vec<usize>,
  table: Vec<Vec<usize>>
}

impl LCA {
  fn new(g:Vec<Vec<usize>>, n:usize, root:usize, max:usize) -> LCA {
    let mut parents = vec![root;n];
    let mut depths = vec![0;n];
    let mut table = vec![vec![0;n];max];
    LCA::dfs(&g, &mut parents, &mut depths, root);
    LCA::doubling(&mut table, &parents);
    LCA { depths, table }
  }
  fn dfs(g: &Vec<Vec<usize>>, parents: &mut Vec<usize>, depths: &mut Vec<usize>, u:usize) {
    for &v in &g[u] {
      if v == parents[u] { continue }
      parents[v] = u;
      depths[v] = depths[u] + 1;
      LCA::dfs(g, parents, depths, v);
    }
  }
  fn doubling(table: &mut Vec<Vec<usize>>, parents: &Vec<usize>) {
    let max = table.len();
    let n = parents.len();
    for (i, &v) in parents.iter().enumerate() {
      table[0][i] = v;
    }
    for i in 1..max {
      for j in 0..n {
        table[i][j] = table[i-1][table[i-1][j]];
      }
    }
  }
  fn lca(&self, mut u:usize, mut v:usize) -> usize {
    if self.depths[u] > self.depths[v] {
      std::mem::swap(&mut u, &mut v);
    }
    let diff = self.depths[v] - self.depths[u];
    let max = self.table.len();
    for i in 0..max {
      if diff >> i == 0 { break }
      else if diff >> i & 1 == 1 {
        v = self.table[i][v];
      }
    }
    if u == v { u } 
    else {
      for i in (0..max).rev() {
        if self.table[i][u] != self.table[i][v] {
          u = self.table[i][u];
          v = self.table[i][v];
        }
      }
      self.table[0][u]
    }
  }
  fn distance(&self, u:usize, v:usize) -> usize {
    let lca_index = self.lca(u, v);
    self.depths[u] + self.depths[v] - 2 * self.depths[lca_index]
  }
}

fn main() {
  input! {
    n:usize,
    edges:[(Usize1,Usize1);n-1],
    q:usize,
    queries:[(Usize1,Usize1);q]
  }

  let mut g = vec![vec![];n];
  for (a,b) in edges {
    g[a].push(b);
    g[b].push(a);
  }
  
  let lca = LCA::new(g, n, 0, 25);
  for (a, b) in queries {
    println!("{}", lca.distance(a,b) + 1);
  }
}
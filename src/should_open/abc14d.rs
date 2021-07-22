/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

struct LCA {
  graph: Vec<Vec<usize>>,  // 木を表現する隣接行列
  parent: Vec<Vec<usize>>, // parent[k][u] := uの2^k先の親
  dist: Vec<usize>,        // rootからの距離
  default_value: usize     // parentやdistに入る初期値
}

impl LCA {
  fn new(
    graph: Vec<Vec<usize>>,
    default_value: usize,
  ) -> LCA {
    let v = graph.len();
    let mut k = 1;
    while (1 << k) < v {
      k += 1;
    }
    let parent = vec![vec![default_value;v];k];
    let dist = vec![default_value;v];
    LCA {
      graph, parent, dist, default_value
    }
  }

  fn init(
    &mut self,
    root: usize
  ) {
    self.dfs(root, self.default_value, 0);
    let k = self.parent.len();
    let n = self.graph.len();
    for i in 0..k-1 {
      for j in 0..n {
        if self.parent[i][j] == self.default_value {
          self.parent[i+1][j] = self.default_value;
        } else {
          self.parent[i+1][j] = self.parent[i][self.parent[i][j]];
        }
      }
    }
  }

  fn dfs(&mut self, v:usize, p:usize, d:usize) {
    self.parent[0][v] = p;
    self.dist[v] = d;
    for i in 0..self.graph[v].len() {
      let e = self.graph[v][i];
      if e != p {
        self.dfs(e, v, d+1);
      }
    }
  }

  fn query(&self, mut u:usize, mut v:usize) -> usize {
    if self.dist[u] < self.dist[v] {
      std::mem::swap(&mut u, &mut v);
    }
    let k = self.parent.len();
    // LCAまでの距離を同じにする
    for i in 0..k {
      if (self.dist[u] - self.dist[v]) >> i & 1 == 1 {
        u = self.parent[i][u];
      }
    }
    // LCAを求める
    // 二分探索でもできる様子(そっちの方が早いと思われる)
    if u == v { return u }
    for i in (0..k).rev() {
      if self.parent[i][u] != self.parent[i][v] {
        u = self.parent[i][u];
        v = self.parent[i][v];
      }
    }
    self.parent[0][u]
  }
}

pub fn main(
) {
  input! {
    n:usize,
    edges:[(Usize1, Usize1);n-1],
    q:usize,
    queries:[(Usize1, Usize1);q]
  }

  let mut g = vec![vec![];n];
  for (f, t) in edges {
    g[f].push(t);
    g[t].push(f);
  }

  let mut lca = LCA::new(g, 1_000_000_000_000usize);
  lca.init(0);

  for (a, b) in queries {
    let pi = lca.query(a, b);
    let lcav = lca.dist[pi];
    println!("{}", lca.dist[a] + lca.dist[b] + 1 - 2 * lcav);
  }
}

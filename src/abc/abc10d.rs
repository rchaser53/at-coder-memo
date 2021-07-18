/* OUTPUT FILE */

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

// 蟻本(p190)
#[derive(Clone, Copy, Eq, PartialEq)]
struct Edge {
  to: usize,
  cap: isize,
  rev: usize
}

struct FordFulkerson {
  g: Vec<Vec<Edge>>,
  used: Vec<bool>
}

impl FordFulkerson {
  fn new(n:usize) -> FordFulkerson {
    let mut g = vec![];
    let mut used = vec![];
    for i in 0..n {
      g.push(vec![]);
      used.push(false);
    }
    FordFulkerson { g, used }
  }

  fn add_edge(&mut self, from:usize, to:usize, cap:isize) {
    let to_edge = Edge {
      to, cap, rev: self.g[to].len()
    };
    self.g[from].push(to_edge);
    let from_edge = Edge {
      to: from, cap: 0, rev: self.g[from].len() - 1
    };
    self.g[to].push(from_edge);
  }

  fn dfs(&mut self, v:usize, t:usize, f:isize) -> isize {
    if v == t {
      return f
    }

    self.used[v] = true;
    for i in 0..self.g[v].len() {
      let edge = self.g[v][i];
      if !self.used[edge.to] && 0 < edge.cap {
        let d = self.dfs(edge.to, t, std::cmp::min(f, edge.cap));
        if 0 < d {
          self.g[v][i].cap -= d;
          self.g[edge.to][edge.rev].cap += d;
          return d
        }
      }
    }
    0
  }

  fn max_flow(&mut self, s:usize, t:usize) -> isize {
    let mut flow = 0;
    loop {
      self.used = vec![false; self.used.len()];
      let f = self.dfs(s, t, isize::max_value());
      if f == 0 {
        return flow
      }
      flow += f;
    }
  }
}

pub fn main(
) {
  input! {
    n:usize,
    g:usize,
    e:usize,
    targets:[usize;g],
    edges:[(usize, usize);e]
  }

  let v = n + 1;
  let mut ff = FordFulkerson::new(v);
  for p in targets {
    ff.add_edge(p, n, 1);
  }

  for (from, to) in edges {
    ff.add_edge(from, to, 1);
    ff.add_edge(to, from, 1);
  }

  println!("{}", ff.max_flow(0, n));
}

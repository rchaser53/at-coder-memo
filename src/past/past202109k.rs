/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::Ordering;

type Cost = isize;
type Flow = isize;
const INF:Cost = 1isize << 50;
#[derive(Clone)]
struct Edge {
  to: usize,
  rev: usize,
  cap: Flow,
  cost: Cost,
  is_reversed: bool
}
impl Edge {
  pub fn new(to:usize, cap:Flow, cost:Cost, rev:usize, is_reversed:bool) -> Self {
    Edge {
      to,
      cap,
      cost,
      rev,
      is_reversed
    }
  }
}

struct PrimalDual {
  graph: Vec<Vec<Edge>>,
  potential: Vec<Cost>,
  min_cost: Vec<Cost>,
  pre_v: Vec<Option<usize>>,
  pre_e: Vec<Option<usize>>
}
impl PrimalDual {
  pub fn new(n:usize) -> Self {
    PrimalDual {
      graph: vec![vec![];n],
      potential: vec![0;n],
      min_cost: vec![INF;n],
      pre_v: vec![None;n],
      pre_e: vec![None;n]
    }
  }
  pub fn add_edge(&mut self, from:usize, to:usize, cap:Flow, cost:Cost) {
    let to_rev = self.graph[to].len();
    let from_rev = self.graph[from].len();
    self.graph[from].push(Edge::new(to, cap, cost, to_rev, false));
    self.graph[to].push(Edge::new(from, 0, -cost, from_rev, true));
  }
  pub fn init(&mut self) {
    self.potential.iter_mut().for_each(|x| *x = 0);
    self.pre_v.iter_mut().for_each(|x| *x = None);
    self.pre_e.iter_mut().for_each(|x| *x = None);
  }
  pub fn min_cost_flow(&mut self, s:usize, t:usize, mut f:Flow) -> Option<Cost> {
    self.init();
    let n = self.graph.len();
    let mut que = BinaryHeap::new();
    let mut result = 0;
    while 0 < f {
      self.min_cost.iter_mut().for_each(|x| *x = INF);
      self.min_cost[s] = 0;
      que.push(Reverse((0, s)));
      while let Some(p) = que.pop() {
        let (cost, pos) = p.0;
        if self.min_cost[pos] < cost {
          continue
        }
        for (i, e) in self.graph[pos].iter().enumerate() {
          let next_cost = self.min_cost[pos] + e.cost + self.potential[pos] - self.potential[e.to];
          if 0 < e.cap && next_cost < self.min_cost[e.to] {
            self.min_cost[e.to] = next_cost;
            self.pre_v[e.to] = Some(pos);
            self.pre_e[e.to] = Some(i);
            que.push(Reverse((next_cost, e.to)));
          }
        }
      }
      if self.min_cost[t] == INF {
        return None;
      }
      (0..n).for_each(|i| self.potential[i] += self.min_cost[i]);
      let mut add_flow = f;
      {
        let mut v = t;
        while v != s {
          let pre_v = self.pre_v[v].unwrap();
          let pre_e = self.pre_e[v].unwrap();
          add_flow = std::cmp::min(add_flow, self.graph[pre_v][pre_e].cap);
          v = pre_v;
        }
        f -= add_flow;
      }
      result += add_flow * self.potential[t];
      {
        let mut v = t;
        while v != s {
          let pre_v = self.pre_v[v].unwrap();
          let pre_e = self.pre_e[v].unwrap();
          self.graph[pre_v][pre_e].cap -= add_flow;
          let rev = self.graph[pre_v][pre_e].rev;
          self.graph[v][rev].cap += add_flow;
          v = pre_v;
        }
      }
    }
    Some(result)

  }
}



fn main() {
  input!{
    p:usize,
    q:usize,
    rows:[Chars;p],
    aps:[(isize,isize);p],
    bps:[(isize,isize);q],
  }
  let inf = 1 << 40;

  let mut is_goods = vec![vec![];p];
  for i in 0..p {
    let row = &rows[i];
    for c in row {
      is_goods[i].push(*c == '1');
    }
  }

  let mut min_cost_flow = PrimalDual::new(p+q+2);
  let src = p + q;
  let dest = src+1;
  for i in 0..p {
    let (a,b) = aps[i];
    let from = i;
    for j in 0..q {
      if is_goods[i][j] {
        let (c,d) = bps[j];
        let to = p + j;
        min_cost_flow.add_edge(from, to, 1, inf - (a-b+c-d));
      }
    }
  }
  for i in 0..p {
    min_cost_flow.add_edge(src, i, 1, 0);
  }
  for i in 0..q {
    min_cost_flow.add_edge(i+p, dest, 1, 0);
  }
  let pq = std::cmp::min(p, q) as isize;
  min_cost_flow.add_edge(src, dest, pq, inf);
  let flow = min_cost_flow.min_cost_flow(src, dest, pq).unwrap();
  let mut sum = 0;
  for (_, b) in &aps {
    sum += b;
  }
  for (_, d) in &bps {
    sum += d;
  }
  sum += inf * pq - flow;
  
  println!("{}", sum);
}
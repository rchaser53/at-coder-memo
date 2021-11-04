/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn dijkstra(
  n: usize,
  default_val: usize,
  graph: &Vec<Vec<(usize, usize)>>,
  start: usize,
) -> Vec<(usize, HashSet<usize>)> {
  let mut score = vec![(default_val,HashSet::new());n];
  let mut pq = BinaryHeap::new();
  score[start] = (0, HashSet::new());
  pq.push(std::cmp::Reverse((0, start)));
  while let Some(std::cmp::Reverse((cv, ci))) = pq.pop() {
    if score[ci].0 < cv {
      continue
    }
    
    for &(av, ni) in &graph[ci] {
      let nv = av + cv;
      if nv < score[ni].0 {
        let mut set = HashSet::new();
        set.insert(ci);
        score[ni] = (nv, set);
        pq.push(std::cmp::Reverse((nv, ni)));
      } else if nv == score[ni].0 {
        score[ni].1.insert(ci);
      }
    }
  }
  score
}

fn main() {
  input! {
    n:usize,
    m:usize,
    vals:[usize;n],
    edges:[(Usize1,Usize1,usize);m]
  }

  let mut g = vec![vec![];n];
  for (a, b, v) in edges {
    g[a].push((v, b));
    g[b].push((v, a));
  }

  let inf = usize::max_value();
  let shortest = dijkstra(n, inf, &g, 0);
  let mut g = vec![vec![];n];
  for i in 0..n {
    let (_, set) = &shortest[i];
    for &v in set {
      g[v].push(i);
    }
  }

  let mut result = vec![0;n];
  result[0] = vals[0];
  let mut stack = vec![(vals[0], 0)];
  while !stack.is_empty() {
    let mut new_stack = vec![];
    while let Some((cv, ci)) = stack.pop() {
      for &ni in &g[ci] {
        let nv = cv + vals[ni];
        if result[ni] < nv {
          result[ni] = nv;
          new_stack.push((nv, ni));
        }
      }
    }
    stack = new_stack;
  }

  println!("{}", result[n-1]);
}
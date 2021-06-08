/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use proconio::{input};
use proconio::marker::*;
use std::collections::*;

fn dijkstra(
  n: usize,
  default_val: usize,
  graph: &Vec<Vec<(usize, usize, usize)>>,
  start: usize
) -> Vec<(usize, Vec<usize>)> {
  let mut score = vec![(default_val, vec![]);n];
  let mut pq = BinaryHeap::new();
  score[start].0 = 0;
  pq.push(std::cmp::Reverse((0, start, vec![])));
  while let Some(std::cmp::Reverse((w_u, u, stack))) = pq.pop() {
    if score[u].0 < w_u {
      continue
    }
    
    for &(v, w_v, v_num) in graph[u].iter() {
      let w = w_u + w_v;
      if w < score[v].0 {
        let mut new_stack = stack.clone();
        new_stack.push(v_num);
        score[v]= (w, new_stack.clone());
        pq.push(std::cmp::Reverse((w, v, new_stack)));
      }
    }
  }
  score
}
 
fn adjacency_list(
  n: usize,
  uvw: &Vec<(usize, usize, usize)>
) -> Vec<Vec<(usize, usize, usize)>> {
  let mut e = vec![vec![]; n];
  let mut count = 0;
  for &(u, v, w) in uvw.iter() {
    e[u].push((v, w, count));
    count += 1;
    e[v].push((u, w, count));
    count += 1;
  }
  e
}

pub fn main(
) {
  input! {
    n:usize,
    m:usize,
    vals:[(Usize1, Usize1, usize);m]
  }

  let inf = 1_000_000_000_000usize;
  let g = adjacency_list(n, &vals);
  let mut set = HashSet::new();
  for i in 0..n {
    let result = dijkstra(n, inf, &g, i);
    for (_, vs) in result {
      for v in vs {
        set.insert(v);
      }
    }
  }

  println!("{}", m - set.len() / 2);
}
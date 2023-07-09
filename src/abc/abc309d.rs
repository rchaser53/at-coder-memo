/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn dijkstra(
  n: usize,
  default_val: usize,
  graph: &Vec<Vec<(usize, usize)>>,
  start: usize
) -> Vec<usize> {
  let mut score = vec![default_val;n];
  let mut pq = BinaryHeap::new();
  score[start] = 0;
  pq.push(std::cmp::Reverse((0, start)));
  while let Some(std::cmp::Reverse((w_u, u))) = pq.pop() {
    if score[u] < w_u {
      continue
    }
    
    for &(v, w_v) in graph[u].iter() {
      let w = w_u + w_v;
      if w < score[v] {
        score[v] = w;
        pq.push(std::cmp::Reverse((w, v)));
      }
    }
  }
  score
}
 

fn main() {
  input! {
    n1:usize,
    n2:usize,
    m:usize,
    ab:[(Usize1,Usize1);m]
  }

  let mut g1 = vec![vec![];n1+n2];
  for i in 0..m {
    let (a,b) = ab[i];
    g1[a].push((b,1));
    g1[b].push((a,1));
  }

  let default_val = 1_000_000_000;
  let s1 = dijkstra(n1+n2, default_val, &g1, 0);
  let s2 = dijkstra(n1+n2, default_val, &g1, n1+n2-1);
  let mut m1 = 0;
  let mut m2 = 0;
  for v in s1 {
    if v != default_val {
      m1 = m1.max(v);
    }
  }
  for v in s2 {
    if v != default_val {
      m2 = m2.max(v);
    }
  }
  println!("{}", m1+m2+1);
}
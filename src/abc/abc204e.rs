/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

fn culc(a:usize) -> usize {
  let mut left = 0;
  let mut right = 1_000_000_100;
  while left + 1 < right {
    let x = (left + right) / 2;
    let v = x * (x + 1);
    if a < v {
      right = x;
    } else {
      left = x;
    }
  }
  left
}

fn dijkstra(
  n: usize,
  default_val: usize,
  graph: &Vec<Vec<(usize, usize, usize, usize)>>,
  start: usize
) -> Vec<usize> {
  let mut score = vec![default_val;n];
  let mut pq = BinaryHeap::new();
  score[start] = 0;
  pq.push(std::cmp::Reverse((0, start)));
  while let Some(std::cmp::Reverse((now, ci))) = pq.pop() {
    if score[ci] < now {
      continue
    }
    
    for &(ni, base_cost, dynamic_cost, shoud_wait_time) in graph[ci].iter() {
      let now = std::cmp::max(now, shoud_wait_time);
      let nv = now + base_cost + dynamic_cost / (now + 1);
      if nv < score[ni] {
        score[ni] = nv;
        pq.push(std::cmp::Reverse((nv, ni)));
      }
    }
  }
  score
}

fn main() {
  input! {
    n:usize,
    m:usize,
    mut edges:[(Usize1,Usize1,usize,usize);m]
  }

  let mut g = vec![vec![];n];
  for (a,b,v,t) in edges {
    let tv = culc(t);
    g[a].push((b,v,t,tv));
    g[b].push((a,v,t,tv));
  }

  let inf = 10usize.pow(15) + 10;
  let score = dijkstra(n, inf, &g, 0);
  if score[n-1] == inf {
    println!("-1");
  } else {
    println!("{}", score[n-1]);
  }
}
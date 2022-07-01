/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;
use std::char::*;

const INF:usize = 1_000_000_000_000_000_000;
fn dijkstra(
  n: usize,
  default_val: usize,
  graph: &Vec<Vec<(usize, usize)>>,
  start: usize
) -> Vec<(usize, usize)> {
  let mut score = vec![(default_val, default_val);n];
  let mut pq = BinaryHeap::new();
  score[start].0 = 0;
  pq.push(std::cmp::Reverse((0, 0)));
  while let Some(std::cmp::Reverse((v1, ci))) = pq.pop() {
    if score[ci].0 < v1 {
      continue
    }
    
    for &(v2, ni) in graph[ci].iter() {
      let nv = v1 + v2;
      if nv < score[ni].0 {
        score[ni]= (nv, ci);
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
    mut edges:[(Usize1,Usize1,usize);m]
  }

  let mut map = HashMap::new();
  let mut g = vec![vec![];n];
  for i in 0..m {
    if edges[i].1 < edges[i].0 {
      let temp = edges[i].1;
      edges[i].1 = edges[i].0;
      edges[i].0 = temp;
    }

    let (a,b,c) = edges[i];
    g[a].push((c, b));
    g[b].push((c, a));

    map.insert((a,b), i);
  }

  let memo = dijkstra(n, INF, &g, 0);
  let mut result = vec![];
  for i in 0..n {
    let (_, ni) = memo[i];
    if ni != INF {
      let (i, ni) = if ni < i {
        (ni, i)
      } else {
        (i, ni)
      };

      if let Some(j) = map.get(&(i, ni)) {
        result.push(j+1);
      }
    }
  }

  println!("{}", result.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}
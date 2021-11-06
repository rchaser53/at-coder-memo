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
) -> Vec<usize> {
  let mut score = vec![default_val;n];
  let mut pq = BinaryHeap::new();
  score[start] = 0;
  pq.push(std::cmp::Reverse((0, start)));
  while let Some(std::cmp::Reverse((cv, ci))) = pq.pop() {
    if score[ci] < cv {
      continue
    }
    
    for &(av, ni) in &graph[ci] {
      let nv = av + cv;
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
    s:Usize1,
    t:Usize1,
    edges:[(Usize1,Usize1,usize,usize);m]
  }
  let inf = usize::max_value();

  let mut g = vec![vec![];n];
  for &(a,b,v,_) in &edges {
    g[a].push((v,b));
    g[b].push((v,a));
  }
  let base = dijkstra(n, inf, &g, s);
  
  let mut g = vec![vec![];n];
  for (a,b,_,v) in edges {
    g[a].push((v, b));
    g[b].push((v, a));
  }
  let sunukes = dijkstra(n, inf, &g, t);

  let mut tot = (vec![0;n]).into_iter().enumerate().collect::<Vec<(usize,usize)>>();
  for i in 0..n {
    tot[i].1 = base[i] + sunukes[i];
  }
  tot.sort_by(|a,b| a.1.cmp(&b.1));
  let mut tot = tot.into_iter().collect::<VecDeque<(usize,usize)>>();

  let mut result = vec![0;n];
  result[0] = tot[0].1;

  for i in 1..n {
    let ti = i - 1;
    while tot[0].0 <= ti {
      tot.pop_front();
    }
    result[i] = tot[0].1;
  }

  for v in result {
    println!("{}", 10usize.pow(15)-v);
  }
}
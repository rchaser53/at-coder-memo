/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

fn dijkstra(
  n: usize,
  default_val: isize,
  graph: &Vec<Vec<(usize, isize)>>,
  start: usize
) -> Vec<isize> {
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

// 負の辺を扱えるベルマンフォードだと間に合わないのでダイクストラを工夫して使う
// 今回のケースの場合、マイナスの値をコストと考える
// ダイクストラは対象の点までの最低コストを導き出せる
// マイナスの値を加工することで、開始点と最終地点が分かればプラスを一意に定めることができるので
// 制限時間内に求めることができる
fn main() {
  input! {
    n:usize,
    m:usize,
    hs:[isize;n],
    edges:[(Usize1,Usize1);m]
  }

  let mut g = vec![vec![];n];

  for (a,b) in edges {
    let av = hs[a];
    let bv = hs[b];

    if av < bv {
      g[a].push((b, bv - av));
      g[b].push((a, 0));
    } else if av == bv {
      g[a].push((b,0));
      g[b].push((a,0));
    } else {
      g[a].push((b, 0));
      g[b].push((a, av - bv));
    }
  }

  let inf = 10isize.pow(16);
  let result = dijkstra(n, inf, &g, 0);
  let mut max = 0;
  for i in 1..n {
    max = std::cmp::max(max, hs[0] - hs[i] - result[i]);
  }

  println!("{}", max);
}
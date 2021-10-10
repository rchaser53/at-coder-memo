use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use std::cmp::Reverse;

const MOD:usize = 998244353;

fn dijkstra(
  n: usize,
  default_val: usize,
  graph: &Vec<Vec<(usize, usize)>>,
  start: usize,
  prev: &mut Vec<usize>,
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
        prev[ni] = ci;
        pq.push(std::cmp::Reverse((nv, ni)));
      }
    }
  }
  score
}

fn main() {
  input!{
    n:usize,
    m:usize,
    k:isize,
    ai:[Usize1;m],
    edges:[(Usize1,Usize1);n-1]
  }

  let mut g = vec![vec![];n];
  let mut map = HashMap::new();
  for (mut a, mut b) in edges {
    g[a].push((1, b));
    g[b].push((1, a));

    if b < a {
      std::mem::swap(&mut a, &mut b);
    }
    // 重複する辺や自己ループがある場合は別途ケアすること
    map.insert((a,b), 0);
  }

  // 経路復元
  // 辺( edge )を利用している回数の取得
  for i in 0..m-1 {
    let ci = ai[i];
    let ni = ai[i+1];
    let mut routes = vec![0;n];
    dijkstra(n, 1_000_000_000, &g, ci, &mut routes);

    let mut stack = vec![ni];
    let mut cci = ni;
    while cci != ci {
      cci = routes[cci];
      stack.push(cci);
    }

    for j in 0..stack.len()-1 {
      let mut ci = stack[j];
      let mut ni = stack[j+1];

      if ni < ci {
        std::mem::swap(&mut ci, &mut ni);
      }
      *map.entry((ci, ni)).or_insert(0) += 1;
    }
  }
  
  let mut result_map = HashMap::new();
  result_map.insert(0, 1);

  for (_, v) in map {
    let mut new_result_map = HashMap::new();

    for (nv, nm) in &result_map {
      let entry = new_result_map.entry(nv + v).or_insert(0);
      *entry += nm;
      *entry %= MOD;

      let entry = new_result_map.entry(nv-v).or_insert(0);
      *entry += nm;
      *entry %= MOD;
    }
    result_map = new_result_map;
  }

  if let Some(v) = result_map.get(&k) {
    println!("{}", v);
  } else {
    println!("0");
  }
}
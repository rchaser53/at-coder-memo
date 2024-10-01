/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    uvw:[(Usize1,Usize1,isize);m]
  }

  let mut graph = vec![vec![];n];
  for (u,v,w) in uvw {
    graph[u].push((v,w));
    graph[v].push((u,-w));
  }
  let mut memo = vec![0;n];
  let mut visited = vec![false;n];
  for i in 0..n {
    if visited[i] { continue }
    visited[i] = true;
    let mut que = VecDeque::new();
    que.push_back(i);
    while let Some(v) = que.pop_front() {
      for &(nv,w) in &graph[v] {
        if visited[nv] { continue }
        memo[nv] = memo[v] + w;
        visited[nv] = true;
        que.push_back(nv);
      }
    }
  }

  println!("{}", memo.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}
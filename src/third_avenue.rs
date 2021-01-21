#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

const MOD:usize = 1_000_000_007;
const INF:usize = 1_000_000_000;
fn main() {
  input!{
    h: usize,
    w: usize,
    rows: [Bytes;h]
  }
  
  let mut start = (0, 0);
  let mut goal = (0, 0);
  let mut map = HashMap::new();
  for i in 0..h {
    for ii in 0..w {
      if b'a' <= rows[i][ii] && rows[i][ii] <= b'z' {
        let mut entry = map.entry(rows[i][ii]).or_insert(vec![]);
        entry.push((i, ii));
      } else if b'S' == rows[i][ii] {
        start = (i, ii);
      } else if b'G' == rows[i][ii] {
        goal = (i, ii);
      }
    }
  }
  
  let mut memo = vec![vec![INF;w];h];
  memo[start.0][start.1] = 0;
  let mut seen = HashSet::new();
  let mut que = VecDeque::new();
  que.push_back(start);
  while !que.is_empty() {
    let (r, c) = que.pop_front().unwrap();
    let v = memo[r][c] + 1;
    let mark = rows[r][c];
    
    if let Some(routes) = map.get(&mark) {
      if !seen.contains(&mark) {
        for i in 0..routes.len() {
          let (nr, nc) = routes[i];
          if v < memo[nr][nc] {
            memo[nr][nc] = v;
            que.push_back((nr, nc));        
          } 
        }
      }
    }

    if 0 < r && rows[r-1][c] != b'#' && v < memo[r-1][c] {
      que.push_back((r-1, c));
      memo[r-1][c] = v;
    }
    
    if 0 < c && rows[r][c-1] != b'#' && v < memo[r][c-1] {
      que.push_back((r, c-1));
      memo[r][c-1] = v;
    }

    if r < h-1 && rows[r+1][c] != b'#' && v < memo[r+1][c] {
      que.push_back((r+1, c));
      memo[r+1][c] = v;
    }
    
    if c < w-1 && rows[r][c+1] != b'#' && v < memo[r][c+1] {
      que.push_back((r, c+1));
      memo[r][c+1] = v;
    }
    seen.insert(mark);
  }
  
  if memo[goal.0][goal.1] != INF {
    println!("{}", memo[goal.0][goal.1]);  
  } else {
    println!("-1");
  }
}

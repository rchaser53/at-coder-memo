/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn dijkstra(
  n: usize,
  default_val: isize,
  graph: &Vec<Vec<(isize, usize)>>,
  start: usize
) -> Vec<(isize, isize, usize)> {
  let mut score = vec![(default_val, default_val, default_val as usize);n];
  let mut stack = vec![];
  score[start].0 = 0;
  score[start].1 = 0;
  stack.push((0,0,start));
  while let Some((v1, num, ci)) = stack.pop() {    
    for &(v2, ni) in graph[ci].iter() {
      let nv = v1 + v2;
      let n_num = num+1;
      if n_num < score[ni].1 {
        score[ni]= (nv, n_num, ni);
        stack.push((nv, n_num, ni));
      } else if n_num == score[ni].1 && nv < score[ni].0 {
        score[ni]= (nv, n_num, ni);
        stack.push((nv, n_num, ni));
      }
    }
  }
  score
}

fn main() {
  input! {
    n:usize,
    an:[isize;n],
    ss:[Chars;n],
    q:usize,
    queries:[(Usize1,Usize1);q] 
  }

  let bv = 10isize.pow(9)+10;
  let mut g = vec![vec![];n];
  for i in 0..n {
    for j in 0..n {
      if i == j { continue }
      if ss[i][j] == 'Y' {
        g[i].push((bv-an[j],j));
      }
    }
  }

  let mut memo = vec![vec![];n];
  let default_val = 10isize.pow(15);
  for i in 0..n {
    memo[i] = dijkstra(n, default_val, &g, i);
  }

  for (a, b) in queries {
    if memo[a][b].0 == default_val {
      println!("Impossible");
    } else {
      let num = memo[a][b].1;
      let val = (memo[a][b].0 - num * bv).abs() + an[a];
      println!("{} {}", num, val);
    }
  }
}
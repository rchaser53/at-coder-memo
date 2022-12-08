/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;


fn main() {
  input! {
    n:usize,
    m:usize,
    q:usize,
    edges:[(Usize1,Usize1,isize);m],
    xy:[(Usize1,Usize1);q]
  }

  let mut g = vec![vec![];n];
  for (a,b,c) in edges {
    g[a].push((b,c));
    g[b].push((a,-c));
  }

  let mut labels = vec![None;n];
  let mut inf = vec![false;n];

  for i in 0..n {
    if labels[i].is_some() { continue }
    let mut que = VecDeque::new();
    labels[i] = Some((i,0));
    que.push_back(i);
    while let Some(u) = que.pop_front() {
      if let Some((_, base)) = labels[u] {
        for &(v, cost) in &g[u] {
          let nv = base + cost;
          if let Some((_, h)) = labels[v] {
            if h != nv {
              inf[i] = true;
            }
            continue
          }
          labels[v] = Some((i,nv));
          que.push_back(v);
        }
      }
    }
  }

  for (x,y) in xy {
    let rx = labels[x].unwrap();
    let ry = labels[y].unwrap();
    if rx.0 != ry.0 {
      println!("nan");
    } else if inf[rx.0] {
      println!("inf")
    } else {
      println!("{}", ry.1-rx.1);
    }
  }
}
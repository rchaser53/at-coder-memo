/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    abxy:[(Usize1,Usize1,isize,isize);m]
  }

  let mut g = vec![vec![];n];
  for (a,b,x,y) in abxy {
    g[a].push((b,x,y));
    g[b].push((a,-x,-y));
  }

  let mut result = vec![None;n];
  result[0] = Some((0,0));

  let mut stack = vec![(0,0,0)];

  while !stack.is_empty() {
    let mut new_stack = vec![];
    while let Some((ci, cx, cy)) = stack.pop() {
      for &(ni, ax, ay) in &g[ci] {
        if result[ni].is_some() { continue }
        let nx = cx + ax;
        let ny = cy + ay;
        result[ni] = Some((nx, ny));
        new_stack.push((ni, nx, ny));
      }
    }
    stack = new_stack;
  }

  for a in result {
    if let Some((x, y)) = a {
      println!("{} {}", x, y);
    } else {
      println!("undecidable");
    }
  }
}

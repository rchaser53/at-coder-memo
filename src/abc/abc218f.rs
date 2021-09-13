use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    n:usize,
    m:usize,
    edges:[(Usize1,Usize1);m]
  }

  let inf = 1_000_000_000;
  let mut g = vec![vec![];n];
  for &(a, b) in &edges {
    g[a].push(b);
  }

  let mut stack = vec![(0,0)];
  let mut base = vec![inf;n];
  let mut prev = vec![inf;n];
  base[0] = 0;
  while !stack.is_empty() {
    let mut new_stack = vec![];
    while let Some((ci, v)) = stack.pop() {
      let nv = v + 1;
      for &ni in &g[ci] {
        if nv < base[ni] {
          base[ni] = nv;
          new_stack.push((ni, nv));
          prev[ni] = ci;
        }
      }
    }
    stack = new_stack;
  }

  let mut route = vec![n-1];
  let mut si = n-1;
  while si != 0 && si != inf {
    si = prev[si];
    route.push(si);
  }
  route.reverse();


  let mut set = HashSet::new();
  for i in 0..route.len()-1 {
    set.insert((route[i], route[i+1]));
  }

  for (from, to) in edges {
    if !set.contains(&(from, to)) {
      if base[n-1] == inf {
        println!("-1");
      } else {
        println!("{}", base[n-1]);
      }
      continue
    }

    let mut stack = vec![(0,0)];
    let mut result = vec![inf;n];
    result[0] = 0;
    while !stack.is_empty() {
      let mut new_stack = vec![];
      while let Some((ci, v)) = stack.pop() {
        let nv = v + 1;
        for &ni in &g[ci] {
          if ci == from && ni == to {
            continue
          }
  
          if nv < result[ni] {
            result[ni] = nv;
            new_stack.push((ni, nv));
          }
        }
      }
      stack = new_stack;
    }

    if result[n-1] == inf {
      println!("-1")
    } else {
      println!("{}", result[n-1]);
    }
  }
}
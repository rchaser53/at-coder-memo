/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

const MOD:usize = 1_000_000_007;

pub fn main(
) {
  input! {
    n:usize,
    edges:[(Usize1,Usize1);n-1],
    mut scores:[usize;n]
  }
  scores.sort();
  scores.reverse();
 
  let mut g = vec![(vec![], 0);n];
  for &(a, b) in &edges {
    g[a].0.push(b);
    g[b].0.push(a);
  }

  let mut i = 0;
  let mut stack = vec![(0,1_000_000_000)];
 
  while !stack.is_empty() {
    let mut new_stack = vec![];
    while let Some((ci, li)) = stack.pop() {
      g[ci].1 = scores[i];
      i += 1;
 
      for &ni in &g[ci].0 {
        if ni == li { continue }
        new_stack.push((ni, ci));
      }
    }
    stack = new_stack;
  }
  
  let mut result = 0usize;
  for (a, b) in edges {
    result += std::cmp::min(g[a].1, g[b].1);
  }
  
  println!("{}", result);
  println!("{}", g.iter().map(|v| v.1.to_string()).collect::<Vec<String>>().join(" "));
}

/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;

fn main() { 
  input! { 
    n:usize,
    edges:[(Usize1,Usize1);n],
    q:usize,
    queries:[(Usize1,Usize1);q]
  }

  // ループ検出
  let mut g = vec![HashSet::new();n];
  for (a, b) in edges {
    g[a].insert(b);
    g[b].insert(a);
  }

  let mut stack = vec![];
  for i in 0..n {
    if g[i].len() == 1 {
      stack.push(i);
    }    
  }

  let mut cg = g.clone();
  let mut loop_inedes = vec![true;n];
  while let Some(i) = stack.pop() {
    loop_inedes[i] = false;
    let ti = *cg[i].iter().next().unwrap();
    cg[ti].remove(&i);
    if cg[ti].len() == 1 {
      stack.push(ti);
    }
  }
  // ループ検出

  let mut dict = vec![0;n];
  for i in 0..n {
    if loop_inedes[i] {
      dict[i] = i;
      for &j in &g[i] {
        if !loop_inedes[j] {
          let mut stack = vec![(j,i)];
          
          while !stack.is_empty() {
            let mut new_stack = vec![];
            while let Some((ci, last)) = stack.pop() {
              dict[ci] = i;
              for &ni in &g[ci] {
                if last == ni { continue }
                new_stack.push((ni, ci));
              }
            }
            stack = new_stack;
          }
        }
      }
    }
  }

  for (a, b) in queries {
    if dict[a] == dict[b] {
      println!("Yes");
    } else {
      println!("No");
    }
  }
}
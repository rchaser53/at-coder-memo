/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;

struct Helper {
  memo: Vec<isize>,
  g: Vec<Vec<usize>>,
  li: usize,
  si: usize,
}

impl Helper {
  fn dfs(&mut self, v:isize, ci:usize, last:usize) {
    if self.memo[ci] >= 0 {
      self.li = ci;
      self.si = last;
      return
    }
    self.memo[ci] = v;
    for i in 0..self.g[ci].len() {
      let ni = self.g[ci][i];
      if ni == last { continue }
      self.dfs(v+1, ni, ci);
    }
  }
}

fn main() { 
  input! { 
    n:usize,
    edges:[(Usize1,Usize1);n],
    q:usize,
    queries:[(Usize1,Usize1);q]
  }

  let mut g = vec![vec![];n];
  for (a, b) in edges {
    g[a].push(b);
    g[b].push(a);
  }

  let inf = 1_000_000_000;
  let mut helper = Helper { memo: vec![-1;n], g, li:0, si:0 };  
  helper.dfs(0, 0, inf);
  let memo = helper.memo;
  let mut ci = helper.li;
  let sv = memo[helper.si];
  let g = helper.g;

  let mut loop_inedes = vec![false;n];
  loop_inedes[ci] = true;
  let mut cv = memo[ci];
  while sv < cv {
    for &ni in &g[ci] {
      if memo[ni] == cv - 1 {
        loop_inedes[ni] = true;
        ci = ni;
        break
      }
    }
    cv -= 1;
  }

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
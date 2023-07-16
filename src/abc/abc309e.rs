/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    p:[Usize1;n-1],
    xy:[(Usize1,isize);m]
  }

  let mut map = HashMap::new();
  for (x,mut y) in xy {
    let entry = map.entry(x).or_insert(0);
    *entry = *entry.max(&mut y);
  }

  let mut g = vec![vec![];n];
  for i in 0..n-1 {
    g[p[i]].push(i+1);
  }

  let mut roots = vec![true;n];
  for children in &g {
    for &i in children {
      roots[i] = false;
    }
  }

  let mut seen = vec![false;n];
  for i in 0..n {
    if !roots[i] { continue }

    let mut stack = vec![];
    if let Some(&num) = map.get(&i) {
      stack.push((i,num));
    } else {
      stack.push((i,-1));
    }

    while !stack.is_empty() {
      let mut new_stack = vec![];
      while let Some((ci, cv)) = stack.pop() {
        if 0 <= cv {
          seen[ci] = true;
        } 

        let nv = cv - 1;
        for &ni in &g[ci] {
          let nv = if let Some(&num) = map.get(&ni) {
            nv.max(num)
          } else {
            nv
          };

          new_stack.push((ni, nv));
        }
      }
      stack = new_stack;
    }
  }

  let mut result = 0;
  for v in seen {
    if v {
      result += 1;
    }
  }
  println!("{}", result);
}
/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn helper(rows:&Vec<Vec<char>>, i:usize, j:usize) -> usize {
  let h = rows.len();
  let w = rows[0].len();

  if rows[i][j] == '#' { return 1 }
  if 0 < i && rows[i-1][j] == '#' { return 2 }
  if i < h-1 && rows[i+1][j] == '#' { return 2 }
  if 0 < j && rows[i][j-1] == '#' { return 2 }
  if j < w-1 && rows[i][j+1] == '#' { return 2 }
  0
}
 
fn main() {
  input! {
    h:usize,
    w:usize,
    s:[Chars;h]
  }

  let mut seen = vec![vec![false;w];h];
  let mut result = 0usize;
  for i in 0..h {
    for j in 0..w {
      if seen[i][j] { continue }

      let t = helper(&s, i, j);
      if t == 1 { continue }
      if t == 2 { result = result.max(1); continue }

      let mut set = HashSet::new();
      set.insert((i,j));
      seen[i][j] = true;
      let mut stack = vec![(i,j)];
      while !stack.is_empty() {
        let mut new_stack = vec![];
        while let Some((ci,cj)) = stack.pop() {
          if 0 < ci  {
            let (ni, nj) = (ci-1, cj);
            let t = helper(&s,ni,nj);
            if t != 1 {
              set.insert((ni,nj));
            }
            if !seen[ni][nj] && t == 0  {
              new_stack.push((ni, nj));
              seen[ni][nj] = true;
            }
          }

          if ci < h-1 {
            let (ni, nj) = (ci+1, cj);
            let t = helper(&s,ni,nj);
            if t != 1 {
              set.insert((ni,nj));
            }
            if !seen[ni][nj] && t == 0  {
              new_stack.push((ni, nj));
              seen[ni][nj] = true;
            }
          }

          if 0 < cj {
            let (ni, nj) = (ci, cj-1);
            let t = helper(&s,ni,nj);
            if t != 1 {
              set.insert((ni,nj));
            }
            if !seen[ni][nj] && t == 0  {
              new_stack.push((ni, nj));
              seen[ni][nj] = true;
            }
          }

          if cj < w-1 {
            let (ni, nj) = (ci, cj+1);
            let t = helper(&s,ni,nj);
            if t != 1 {
              set.insert((ni,nj));
            }
            if !seen[ni][nj] && t == 0  {
              new_stack.push((ni, nj));
              seen[ni][nj] = true;
            }
          }
        }
        stack = new_stack;
      }

      result = result.max(set.len());
    }
  }

  println!("{}", result);
}
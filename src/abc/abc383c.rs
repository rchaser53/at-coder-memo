/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    h:usize,
    w:usize,
    d:i32,
    rows:[Chars;h]
  }

  let mut memo = vec![vec![-1;w];h];
  let mut stack = vec![];
  for i in 0..h {
    for j in 0..w {
      if rows[i][j] == 'H' {
        memo[i][j] = d;
        stack.push((i,j,d));
      }
    }
  }

  while !stack.is_empty() {
    let mut new_stack = vec![];
    while let Some((ci,cj,cv)) = stack.pop() {
      let nv = cv-1;
      if 0 < ci && memo[ci-1][cj] < nv && rows[ci-1][cj] != '#' {
        memo[ci-1][cj] = nv;
        if nv > 0 {
          new_stack.push((ci-1,cj,nv));
        }
      }
      if ci < h-1 && memo[ci+1][cj] < nv && rows[ci+1][cj] != '#' {
        memo[ci+1][cj] = nv;
        if nv > 0 {
          new_stack.push((ci+1,cj,nv));
        }
      }

      if 0 < cj && memo[ci][cj-1] < nv && rows[ci][cj-1] != '#'{
        memo[ci][cj-1] = nv;
        if nv > 0 {
          new_stack.push((ci,cj-1,nv));
        }
      }
      if cj < w-1 && memo[ci][cj+1] < nv && rows[ci][cj+1] != '#'{
        memo[ci][cj+1] = nv;
        if nv > 0 {
          new_stack.push((ci,cj+1,nv));
        }
      }
    }
    stack = new_stack;
  }

  let mut result = 0;
  for i in 0..h {
    for j in 0..w {
      if memo[i][j] >= 0 {
        result += 1;
      }
    }
  }
  println!("{}", result);
}
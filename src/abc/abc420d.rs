/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn check(rows: &Vec<Vec<char>>, ni:usize, nj:usize, p:usize) -> usize {
  let c = rows[ni][nj];
  if c == '#' {
    0
  } else {
    if c == 'o' && p % 2 == 1 {
      0
    } else if c == 'x' && p % 2 == 0 {
      0
    } else if c == '?' {
      1
    } else {
      2
    }
  }
}

fn main() {
  input! {
    h:usize,
    w:usize,
    rows:[Chars;h]
  }

  let inf = 1_000_000_000usize;
  let mut memo = vec![vec![vec![inf;w];h];2];

  let mut stack = vec![];
  let mut goal = (0,0);
  for i in 0..h {
    for j in 0..w {
      if rows[i][j] == 'S' {
        stack.push((i,j,0,0));
        memo[0][i][j] = 0;
      }
      if rows[i][j] == 'G' {
        goal = (i,j);
      }
    }
  }

  while !stack.is_empty() {
    let mut new_stack = vec![];
    while let Some((ci,cj,p,cv)) = stack.pop() {
      let nv = cv + 1;
      if 0 < ci {
        let a = check(&rows, ci-1,cj, p);
        let np = (p + a) % 2;
        if memo[np][ci-1][cj] > nv && a > 0 {
          memo[np][ci-1][cj] = nv;
          new_stack.push((ci-1,cj,np,nv));
        }
      }

      if ci < h-1 {
        let a = check(&rows, ci+1, cj, p);
        let np = (p+a) % 2;
        if memo[np][ci+1][cj] > nv  && a > 0 {
          memo[np][ci+1][cj] = nv;
          new_stack.push((ci+1,cj,np,nv));
        }
      }

      if 0 < cj {
        let a = check(&rows, ci, cj-1, p);
        let np = (p+a) % 2;
        if memo[np][ci][cj-1] > nv  && a > 0 {
          memo[np][ci][cj-1] = nv;
          new_stack.push((ci,cj-1,np,nv));
        }
      }

      if cj < w-1 {
        let a = check(&rows, ci, cj+1, p);
        let np = (p+a) % 2;
        if memo[np][ci][cj+1] > nv  && a > 0 {
          memo[np][ci][cj+1] = nv;
          new_stack.push((ci,cj+1,np,nv));
        }
      }
    }
    stack = new_stack;
  }

  if memo[0][goal.0][goal.1] < inf || memo[1][goal.0][goal.1] < inf {
    println!("{}", min(memo[0][goal.0][goal.1], memo[1][goal.0][goal.1]));
  } else {
    println!("-1");
  }
}
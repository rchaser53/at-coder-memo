/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    h:usize,
    w:usize,
    mut rows:[Chars;h]
  }

  let mut s = (0,0);
  let mut g = (0,0);
  for i in 0..h {
    let mut dirty = false;
    for j in 0..w {
      if rows[i][j] == '>' {
        dirty = true;
        rows[i][j] = '#';
        continue
      } else if rows[i][j] != '.' && rows[i][j] != '!' {
        dirty = false;
      }

      if rows[i][j] == 'S' {
        s = (i,j);
      } else if rows[i][j] == 'G' {
        g = (i,j);
      }

      if dirty {
        rows[i][j] = '!';
      }
    }

    let mut dirty = false;
    for j in (0..w).rev() {
      if rows[i][j] == '<' {
        dirty = true;
        rows[i][j] = '#';
        continue
      } else if rows[i][j] != '.' && rows[i][j] != '!' {
        dirty = false;
      }

      if dirty {
        rows[i][j] = '!';
      }
    }
  }

  for i in 0..w {
    let mut dirty = false;
    for j in 0..h {
      if rows[j][i] == 'v' {
        dirty = true;
        rows[j][i] = '#';
        continue
      } else if rows[j][i] != '.' && rows[j][i] != '!' {
        dirty = false;
      }
      if dirty {
        rows[j][i] = '!';
      }
    }

    let mut dirty = false;
    for j in (0..h).rev() {
      if rows[j][i] == '^' {
        dirty = true;
        rows[j][i] = '#';
        continue
      } else if rows[j][i] != '.' && rows[j][i] != '!' {
        dirty = false;
      }

      if dirty {
        rows[j][i] = '!';
      }
    }
  }
  
  let default = 1_000_000_000;
  let mut memo = vec![vec![default;w];h];
  memo[s.0][s.1] = 0;
  let mut stack = vec![(s.0,s.1,0)];
  while !stack.is_empty() {
    let mut new_stack = vec![];
    while let Some((ci,cj,cv)) = stack.pop() {
      let nv = cv+1;
      if 0 < ci && (rows[ci-1][cj] == '.' || rows[ci-1][cj] == 'G') && nv < memo[ci-1][cj] {
        memo[ci-1][cj] = nv;
        new_stack.push((ci-1,cj,nv));
      }

      if ci < h-1 && (rows[ci+1][cj] == '.' || rows[ci+1][cj] == 'G') && nv < memo[ci+1][cj] {
        memo[ci+1][cj] = nv;
        new_stack.push((ci+1,cj,nv));
      }

      if 0 < cj && (rows[ci][cj-1] == '.' || rows[ci][cj-1] == 'G') && nv < memo[ci][cj-1] {
        memo[ci][cj-1] = nv;
        new_stack.push((ci,cj-1,nv));
      }

      if cj < w-1 && (rows[ci][cj+1] == '.' || rows[ci][cj+1] == 'G') && nv < memo[ci][cj+1] {
        memo[ci][cj+1] = nv;
        new_stack.push((ci,cj+1,nv));
      }
    }
    stack = new_stack;
  }

  if memo[g.0][g.1] == default {
    println!("-1");
  } else {
    println!("{}", memo[g.0][g.1]);
  }
}

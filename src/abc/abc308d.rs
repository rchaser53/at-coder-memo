/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn helper(c:char) -> char {
  if c == 's' {
    'n'
  } else if c == 'n' {
    'u'
  } else if c == 'u' {
    'k'
  } else if c == 'k' {
    'e'
  } else {
    's'
  }
}

fn main() {
  input! {
    h:usize,
    w:usize,
    rows: [Chars;h]
  }

  let mut seen = vec![vec![false;w];h];

  if rows[0][0] != 's' {
    println!("No");
    return
  }

  let mut stack = vec![(0,0)];

  seen[0][0] = true;

  while let Some((ci,cj)) = stack.pop() {
    let nv = helper(rows[ci][cj]);
    if 0 < ci && rows[ci-1][cj] == nv && !seen[ci-1][cj] {
      seen[ci-1][cj] = true;
      stack.push((ci-1,cj));
    }

    if ci < h-1 && rows[ci+1][cj] == nv && !seen[ci+1][cj] {
      seen[ci+1][cj] = true;
      stack.push((ci+1,cj));
    }

    if 0 < cj && rows[ci][cj-1] == nv && !seen[ci][cj-1] {
      seen[ci][cj-1] = true;
      stack.push((ci,cj-1));
    }

    if cj < w-1 && rows[ci][cj+1] == nv && !seen[ci][cj+1] {
      seen[ci][cj+1] = true;
      stack.push((ci,cj+1));
    }
  }

  if seen[h-1][w-1] {
    println!("Yes");
  } else {
    println!("No");
  }
}
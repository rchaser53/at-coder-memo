/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::*;

struct Helper {
  rows: Vec<Vec<char>>,
  result: usize,
  k:usize
}

impl Helper {
  fn dfs(&mut self, ci:usize, cj:usize, cv:usize, p:&[char], memo: &mut Vec<Vec<bool>>) {
    if cv == self.k {
      self.result += 1;
      return
    }

    let h = self.rows.len();
    let w = self.rows[0].len();
    let nv = cv + 1;
    for &c in p {
      if c == 'N' {
        if 0 < ci && self.rows[ci-1][cj] != '#' && !memo[ci-1][cj] {
          memo[ci-1][cj] = true;
          self.dfs(ci-1, cj, nv, &['N','W','E'], memo);
          memo[ci-1][cj] = false;
        }
      } else if c == 'S' {
        if ci  < h-1 && self.rows[ci+1][cj] != '#' && !memo[ci+1][cj] {
          memo[ci+1][cj] = true;
          self.dfs(ci+1, cj, nv, &['S','W','E'], memo);
          memo[ci+1][cj] = false;
        }
      } else if c == 'W' {
        if 0 < cj && self.rows[ci][cj-1] != '#' && !memo[ci][cj-1] {
          memo[ci][cj-1] = true;
          self.dfs(ci, cj-1, nv, &['N','S','W'], memo);
          memo[ci][cj-1] = false;
        }
      } else {
        if cj < w-1 && self.rows[ci][cj+1] != '#' && !memo[ci][cj+1] {
          memo[ci][cj+1] = true;
          self.dfs(ci, cj+1, nv, &['N','E','S'], memo);
          memo[ci][cj+1] = false;
        }
      }
    }
  }
}

fn main() {
  input! {
    h:usize,
    w:usize,
    k:usize,
    rows:[Chars;h]
  }

  let mut helper = Helper {
    rows,
    k,
    result: 0
  };
  for i in 0..h {
    for j in 0..w {
      if helper.rows[i][j] == '#' {
        continue
      }
      let mut memo = vec![vec![false;w];h];
      memo[i][j] = true;

      helper.dfs(i,j,0,&['W','E','N','S'], &mut memo);
    }
  }

  println!("{}", helper.result);
}
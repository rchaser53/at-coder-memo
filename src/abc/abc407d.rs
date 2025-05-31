/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

struct Helper {
  rows: Vec<Vec<isize>>,
  result: isize
}

impl Helper {
  fn dfs(&mut self, i:usize, j:usize, set: &mut HashSet<(usize,usize)>) {
    let n = self.rows.len();
    let m = self.rows[0].len();
    if i == n-1 && j == m-1 {
      return
    }

    let (ni,nj) = if j+1 == m {
      (i+1, 0)
    } else {
      (i, j+1)
    };

    self.dfs(ni, nj, set);
    if !set.contains(&(i, j)) {
      set.insert((i,j));
      if i+1 < n && !set.contains(&(i+1, j)) {
        set.insert((i+1, j));
        self.calc(set);
        self.dfs(ni, nj, set);
        set.remove(&(i+1, j));
      }

      if j+1 < m && !set.contains(&(i, j+1)) {
        set.insert((i, j+1));
        self.calc(set);
        self.dfs(ni, nj, set);
        set.remove(&(i, j+1));
      }
      set.remove(&(i, j));
    }   
  }

  fn calc(&mut self, set: &HashSet<(usize, usize)>) {
    let n = self.rows.len();
    let m = self.rows[0].len();
    let mut temp = 0isize;
    for i in 0..n {
      for j in 0..m {
        if !set.contains(&(i, j)) {
          temp ^= self.rows[i][j];
        }
      }
    }
    self.result = max(self.result, temp);
  }
}

fn main() {
  input! {
    h:usize,
    w:usize,
    rows:[[isize;w];h],
  }

  let mut result = 0;
  for i in 0..h {
    for j in 0..w {
      result ^= rows[i][j];
    }
  }
  let mut helper = Helper {
    rows,
    result
  };

  helper.dfs(0, 0, &mut HashSet::new());

  println!("{}", helper.result);
}
/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;


struct Helper {
  success:bool,
  h:usize,
  w:usize
}

impl Helper {
  fn dfs(
    &mut self,
    rows: Vec<Vec<bool>>,
    x:usize,
    y:usize,
  ) {
  
    let mut dirty = false;
    if x < self.w - 1 && rows[y][x+1] {
      dirty = true;
      let mut cloned = rows.clone();
      cloned[y][x+1] = false;
      self.dfs(cloned, x+1, y);
    }

    if y < self.h - 1 && rows[y+1][x] {
      dirty = true;
      let mut cloned = rows.clone();
      cloned[y+1][x] = false;
      self.dfs(cloned, x, y+1);
    }
    
    if !dirty {
      let mut flag = true;
      for i in 0..self.h {
        for j in 0..self.w {
          if rows[i][j] {
            flag = false;
            break
          }
        }
      }

      if flag {
        self.success = true;
      }
    }
  }
}



pub fn main(
) {
  input! {
    h:usize,
    w:usize,
    rows:[Chars;h]
  }
  
  let mut bases = vec![vec![false;w];h];
  for i in 0..h {
    for j in 0..w {
      if rows[i][j] == '#' {
        bases[i][j] = true;
      }
    }
  }

  let mut helper = Helper { success:false, h, w };
  for i in 0..h {
    for j in 0..w {
      let mut cloned = bases.clone();
      cloned[i][j] = false;
      helper.dfs(cloned, j, i);

      if helper.success {
        println!("Possible");
        return
      }
    }
  }
  println!("Impossible");
}

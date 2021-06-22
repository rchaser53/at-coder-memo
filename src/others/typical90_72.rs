/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

struct Helper {
  count:usize,
  h:usize,
  w:usize
}

impl Helper {
  fn culc(
    &mut self,
    rows: Vec<Vec<bool>>,
    now: (usize, usize),
    start: (usize, usize),
    val: usize
  ) {

    let (x, y) = now;
    if now == start && val != 0 {
      if 2 < val {
        self.count = std::cmp::max(self.count, val);
      }
      return
    }

    if 0 < x && rows[y][x-1] {
      let mut new_rows = rows.clone();
      new_rows[y][x-1] = false;
      self.culc(new_rows, (x-1, y), start, val+1);
    }

    if x < self.w-1 && rows[y][x+1] {
      let mut new_rows = rows.clone();
      new_rows[y][x+1] = false;
      self.culc(new_rows, (x+1, y), start, val+1);
    }
    
    if 0 < y && rows[y-1][x] {
      let mut new_rows = rows.clone();
      new_rows[y-1][x] = false;
      self.culc(new_rows, (x, y-1), start, val+1);
    }

    if y < self.h-1 && rows[y+1][x] {
      let mut new_rows = rows.clone();
      new_rows[y+1][x] = false;
      self.culc(new_rows, (x, y+1), start, val+1);
    }
  }
}

pub fn main(
) {
  input! {
    h:usize,
    w:usize,
    vals: [Chars;h]
  }

  let mut base_rows = vec![vec![true;w];h];
  for i in 0..h {
    for j in 0..w {
      base_rows[i][j] = vals[i][j] == '.';
    }
  }

  let mut helper = Helper { count:0, h, w };
  for i in 0..h {
    for j in 0..w {
      helper.culc(base_rows.clone(), (j,i), (j,i), 0);
    }
  }

  if helper.count == 0 {
    println!("-1");
  } else {
    println!("{}", helper.count);
  }
}

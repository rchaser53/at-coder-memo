/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    h:usize,
    w:usize,
    rows:[Chars;h]
  }

  let mut stack = vec![];
  for i in 0..h {
    let mut exist = false;
    let mut temp = vec![];
    for j in 0..w {
      if rows[i][j] == '#' {
        temp.push((i,j));
        exist = true;
      }
    }

    if exist {
      stack.push(temp);
    }
  }
  stack.sort_by(|a,b| a.len().cmp(&b.len()));

  let mut set = HashSet::new();
  for &(_,x) in &stack[0] {
    set.insert(x);
  }

  for &(_,x) in &stack[1] {
    if !set.contains(&x) {
      println!("{} {}", stack[0][0].0+1, x+1);
      return
    }
  }
}
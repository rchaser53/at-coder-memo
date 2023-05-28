/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input!{
    n:usize,
    m:usize,
    mut h:isize,
    k:isize,
    s:Chars,
    xy:[(isize,isize);m],
  }

  let mut set = HashSet::new();
  for (x,y) in xy {
    set.insert((x,y));
  }
  let mut cx = 0;
  let mut cy = 0;

  for c in s {
    if c == 'R' {
      cx += 1; 
    } else if c == 'U' {
      cy += 1;
    } else if c == 'D' {
      cy -= 1;
    } else {
      cx -= 1;
    }
    h -= 1;

    if h < 0 {
      println!("No");
      return
    }

    if set.contains(&(cx,cy)) {
      if h < k {
        h = k;
        set.remove(&(cx,cy));
      }      
    }
  }
  println!("Yes");
}
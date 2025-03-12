/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    mut b:[isize;n],
    mut w:[isize;m],
  }

  w.sort();

  let mut count = 0isize;
  let mut tot = 0;
  let mut minus = vec![];
  for v in b {
    if 0 <= v {
      count += 1;
      tot += v;
    } else {
      minus.push(v);
    }
  }

  let mut is_minus_only = false;
  for i in (0..m).rev() {
    let v = w[i];

    if count == 0 {
      break
    }
    w.pop();
    count -= 1;
    
    if v < 0 {
      is_minus_only = true;
      break
    }

    tot += v;
  }

  if is_minus_only {
    println!("{}", tot);
    return
  }

  minus.sort();
  while !minus.is_empty() && !w.is_empty() {
    let v1 = minus.pop().unwrap();
    let v2 = w.pop().unwrap();
    if v1 + v2 > 0 {
      tot += v1 + v2;
    } else {
      break
    }
  }

  println!("{}", tot);
}
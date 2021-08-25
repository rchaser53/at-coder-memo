/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
    m:usize,
    s:[usize;n],
    t:[usize;m]
  }

  let mut sv = 0;
  let mut tv = 0;
  for &v in &s {
    sv |= 1 << v;
  }
  for &v in &t {
    tv |= 1 << v;
  }

  if (sv == 2 && tv != 2)
  || (sv == 1 && tv != 1)
  {
    println!("-1");
    return
  }

  if sv < 3 {
    println!("{}", t.len());
    return
  }

  let mut last = s[0];
  let mut rv = 0;
  let mut lv = 0;
  for i in 1..n {
    if last != s[i] {
      rv = i;
      break
    }
  }
  for i in (1..n).rev() {
    if last != s[i] {
      lv = n-i;
      break
    }
  }
  let c = std::cmp::min(rv, lv);
  let mut result = 0usize;
  let mut first = true;
  
  for v in t {
    if last != v {
      if first {
        first = false;
        result += c + 1;
      } else {
        result += 2;
      }
    } else {
      result += 1;
    }
    last = v;
  }

  println!("{}", result);
}

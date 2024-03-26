/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::*;
use std::collections::*;

fn helper(a: &Vec<usize> , s: &Vec<char>, i:usize) -> (usize,usize) {
  let mut zv = 0;
  let mut ov = 0;
  if s[i] == '1' {
    zv += a[i];
  } else {
    ov += a[i];
  }

  if s[i+1] == '1' {
    zv += a[i+1];
  } else {
    ov += a[i+1];
  }

  (zv, ov)
}

fn main() {
  input! {
    n:usize,
    s:Chars,
    c:[usize;n]
  }

  if n == 2 {
    let (a,b) = helper(&c, &s, 0);
    println!("{}", a.min(b));
    return
  }

  let mut zo_memo = vec![0;n+1];
  let mut oz_memo = vec![0;n+1];

  for i in 0..n {
    oz_memo[i+1] = oz_memo[i];
    zo_memo[i+1] = zo_memo[i];
    let cc = s[i];
    if i % 2 == 0 {
      if cc == '0' {
        oz_memo[i+1] += c[i];
      } else {
        zo_memo[i+1] += c[i];
      }
    } else {
      if cc == '0' {
        zo_memo[i+1] += c[i];
      } else {
        oz_memo[i+1] += c[i];
      }
    }
  }

  let mut result = 10usize.pow(15);
  for i in 0..n-1 {
    let (zv, ov) = helper(&c, &s, i);

    let l_oz_v = oz_memo[i];
    let l_zo_v = zo_memo[i];
    let r_oz_v = oz_memo[n] - oz_memo[i+2];
    let r_zo_v = zo_memo[n] - zo_memo[i+2];
    if i % 2 == 0 {
      // 01 + 00 + 10
      let v1 = l_zo_v + zv + r_oz_v;
      // 10 + 11 + 01
      let v2 = l_oz_v + ov + r_zo_v;
      result = result.min(v1.min(v2));
    }
    else {
      // 101 + 00 + 101
      let v1 = l_oz_v + zv + r_zo_v;
      // 010 + 11 + 010
      let v2 = l_zo_v + ov + r_oz_v;
      result = result.min(v1.min(v2));
    }
  }

  println!("{}", result);
}
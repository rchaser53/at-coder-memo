/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    k:usize,
    mut s:Chars
  }

  for i in 0..n {
    if s[i] != 'o' {
      continue;
    }
    if i > 0 { s[i - 1] = '.' }
    if i + 1 < n { s[i + 1] = '.' }
  }

  let mut v = vec![];
  let mut p = !0;
  for i in 0..n {
    if s[i] == '?' { continue }
    if p + 1 != i { v.push((p + 1, i)) }
    p = i;
  }
  if p + 1 != n { v.push((p + 1, n)) }
  let m = s.iter().filter(|&&c| c == 'o').count();
  if m == k {
    s = s.iter().map(|&c| if c == '?' {'.'} else {c}).collect();
  } else if m + v.iter().map(|&(l, r)| (r + 1 - l) / 2).sum::<usize>() == k {
    for &(l, r) in v.iter() {
      if (r - l) & 1 == 0 { continue }
      for i in l..r {
        s[i] = if (i - l) & 1 == 0 {'o'} else {'.'}
      }
    }
  }
  println!("{}", s.into_iter().collect::<String>());
}
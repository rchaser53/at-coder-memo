/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    a:[usize;n],
    s:Chars
  }

  let mut result = 0usize;

  let mut memo_l = vec![vec![0;3];n];
  let mut memo_r = vec![vec![0;3];n];

  if s[0] == 'M' {
    memo_l[0][a[0]] += 1;
  }

  if s[n-1] == 'X' {
    memo_r[n-1][a[n-1]] += 1;
  }
  
  for i in 1..n {
    for j in 0..3 {
      memo_l[i][j] = memo_l[i-1][j];
    }
    if s[i] == 'M' {
      memo_l[i][a[i]] += 1;
    }
  }

  for i in (0..n-1).rev() {
    for j in 0..3 {
      memo_r[i][j] = memo_r[i+1][j];
    }
    if s[i] == 'X' {
      memo_r[i][a[i]] += 1;
    }
  }

  for i in 1..n-1 {
    if s[i] != 'E' { continue }
    let mv = a[i];
    for lv in 0..3 {
      let lc = memo_l[i][lv];
      if lc == 0 { continue }

      for rv in 0..3 {
        let rc = memo_r[i][rv];
        if rc == 0 { continue }

        let num = lc * rc;
        let mut temp = vec![true;4];
        temp[mv] = false;
        temp[lv] = false;
        temp[rv] = false;
        for cv in 0..4 {
          if temp[cv] {
            result += cv * num;
            break
          }
        }
      }
    }
  }

  println!("{}", result);
}
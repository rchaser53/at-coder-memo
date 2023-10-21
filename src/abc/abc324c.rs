/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Reverse;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    t:Chars,
    ss:[Chars;n]
  }
  
  let mut result = vec![];
  let tn = t.len();
  for i in 0..n {
    let s = &ss[i];
    let sn = s.len();

    if sn > tn {
      if sn != tn+1 { continue }
      let mut ti = 0;
      let mut si = 0;
      while si < sn && ti < tn {
        if s[si] == t[ti] {
          ti += 1;
        }
        si += 1;     
      }

      if si >= sn-1 && ti == tn {
        result.push(i+1);
      }
    } else if tn > sn {
      if tn != sn+1 { continue }
      let mut ti = 0;
      let mut si = 0;
      while si < sn && ti < tn {
        if s[si] == t[ti] {
          si += 1;
        }
        ti += 1;     
      }

      if si == sn && ti >= tn-1 {
        result.push(i+1);
      }
    } else {
      let mut temp = 0;
      for ii in 0..sn {
        if s[ii] != t[ii] {
          temp += 1;
        }
      }

      if temp <= 1 {
        result.push(i+1);
      }
    }
  }

  println!("{}", result.len());
  println!("{}", result.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}
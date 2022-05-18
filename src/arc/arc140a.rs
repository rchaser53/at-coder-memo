/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

fn main() {
  input! {
    n:usize,
    k:usize,
    s:Chars
  }

  for i in (1..=n).rev() {
    if n % i != 0 {
      continue
    }

    let v = n / i;
    let av = v - 1;
    let mut temp = 0;
    for l in 0..=av {
      let mut map = HashMap::new();
      for j in 0..i {
        let ti = j * v + l;
        *map.entry(s[ti]).or_insert(0) += 1;
      }
      let mut min = 10000;
      for (_, num) in map {
        min = min.min(i-num);
      }
      temp += min;
    }

    if temp <= k {
      println!("{}", v);
      return
    }
  }
  println!("{}", n);
}
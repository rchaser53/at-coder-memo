use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input! {
    n:usize,
    s:Chars,
    q:usize,
    ks:[usize;q]
  }

  for k in ks {
    let (mut d, mut m, mut dm, mut c) = (0usize, 0usize, 0usize, 0usize);
    for i in 0..n {
      match s[i] {
        'D' => d += 1,
        'M' => {
          m += 1;
          dm += d;
        }
        'C' => c += dm,
        _ => {}
      }
      if let Some(j) = i.checked_sub(k-1) {
        match s[j] {
          'D' => {
            dm -= m;
            d -= 1;
          }
          'M' => m -= 1,
          _ => {}
        }
      }
    }
    println!("{}", c);
  }
}
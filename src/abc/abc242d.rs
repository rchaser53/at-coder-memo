/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

fn main() {
  input!{
    s:Chars,
    q:usize,
    vals:[(usize,Usize1);q]
  }

  let mut memo = vec![0;62];
  for i in 0..memo.len() {
    memo[i] = 1 << i;
  }

  for (n, v) in vals {
    let (ci, mut v) = if 60 < n {
      (0, v)
    } else {
      let i = 1 << n;
      (v / i, v % i)
    };

    let mut r = 0;
    let mut i = 60;
    while 0 < v {
      let vv = 1 << i;
      if vv <= v {
        v -= vv;
        r += 1;
      }
      i -= 1;
    }

    let v = (s[ci] as u8 - 'A' as u8) as usize;
    let l = n - r;
    let tot = (l + r * 2 + v) % 3;
    println!("{}", ('A' as u8 + tot as u8) as char);
  }
}
/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input! {
    a:usize,
    b:usize,
    m:usize,
    mut aa:[usize;a],
    mut bb:[usize;b],
    tickets:[(Usize1,Usize1,usize);m]
  }

  let mut pair = vec![0;m];
  for i in 0..m {
    let (ai, bi, v) = tickets[i];
    pair[i] = aa[ai] + bb[bi] - v;
  }

  aa.sort();
  bb.sort();
  pair.push(aa[0]+bb[0]);
  pair.sort();
  println!("{}", pair[0]);
}
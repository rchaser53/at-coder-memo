/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Reverse;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    sc:[(usize,usize);n]
  }
  let mut sc = sc.into_iter().collect::<BTreeMap<usize,usize>>();
  let mut temp = 0;
  while let Some((v,n)) = sc.pop_first() {
    let v2 = v*2;
    let num = n / 2;
    if n % 2 == 1 {
      temp += 1;
    }

    if num != 0 {
      let entry = sc.entry(v2).or_insert(0);
      *entry += num;
    }
  }
  println!("{}", temp);
}
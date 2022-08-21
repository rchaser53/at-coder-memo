/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;


fn helper(a:usize,b:usize,c:usize) -> usize {
  format!("{}{}{}", a,b,c).parse().unwrap()
}

fn main() {
  input! {
    n:usize,
    mut vals:[usize;n]
  }

  vals.sort();
  vals.reverse();

  let mut result = 0;
  let mut a = vec![];
  for i in 0..3 {
    a.push(vals[i]);
  }

  result = result
        .max(helper(a[0],a[1],a[2]))
        .max(helper(a[0],a[2],a[1]))
        .max(helper(a[1],a[0],a[2]))
        .max(helper(a[1],a[2],a[0]))
        .max(helper(a[2],a[0],a[1]))
        .max(helper(a[2],a[1],a[0]));

  println!("{}", result);
}
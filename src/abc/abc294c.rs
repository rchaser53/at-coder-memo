/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input!{
    n:usize,
    m:usize,
    mut a:[usize;n],
    mut b:[usize;m]
  }
  
  let mut ab = vec![];
  for i in 0..n {
    ab.push((a[i],i,true));
  }
  for i in 0..m {
    ab.push((b[i],i, false));
  }

  ab.sort();
  for i in 0..n+m {
    let (_, j, t) = ab[i];
    if t {
      a[j] = i + 1;
    } else {
      b[j] = i + 1;
    }
  }

  println!("{}", a.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
  println!("{}", b.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}
/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

fn helper(a: Ordering) -> Ordering {
  if a == Ordering::Less {
    Ordering::Greater
  } else {
    Ordering::Less
  }
}

fn main() {
  input! {
    n:usize,
    a:[usize;n],
    b:[usize;n]
  }

  let mut memo = vec![];
  for i in 0..n {
    memo.push((i+1, a[i], b[i]));
  }
  memo.sort_by(|a,b| {
    let v = (a.1+a.2).cmp(&(b.1+b.2));
    if v == Ordering::Equal {
      let v = a.1.cmp(&b.1);
      if v == Ordering::Equal {
        a.0.cmp(&b.0)
      } else {
        helper(v)
      }
    } else {
      helper(v)
    }
  });

  let v = memo.into_iter().map(|v| v.0.to_string()).collect::<Vec<String>>();
  println!("{}", v.join(" "));
}
/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;

fn main() {
  input! {
    n:usize,
    x:usize,
    y:usize,
    z:usize,
    a:[usize;n],
    b:[usize;n]
  }

  let mut memo = vec![(0,0,0);n];
  for i in 0..n {
    memo[i] = (a[i],b[i],i);
  }

  let mut result = vec![];
  memo.sort_by(|a,b| {
    let v = a.0.cmp(&b.0);
    if v == Ordering::Equal {
      let v = a.2.cmp(&b.2);
      if v == Ordering::Less {
        Ordering::Greater
      } else {
        Ordering::Less
      }
    } else{
      v
    }
  });
  for _ in 0..x {
    result.push(memo.pop().unwrap().2);
  }

  memo.sort_by(|a,b| {
    let v = a.1.cmp(&b.1);
    if v == Ordering::Equal {
      let v = a.2.cmp(&b.2);
      if v == Ordering::Less {
        Ordering::Greater
      } else {
        Ordering::Less
      }
    } else{
      v
    }
  });
  for _ in 0..y {
    result.push(memo.pop().unwrap().2);
  }

  memo.sort_by(|a,b| {
    let v = (a.0+a.1).cmp(&(b.1+b.0));
    if v == Ordering::Equal {
      let v = a.2.cmp(&b.2);
      if v == Ordering::Less {
        Ordering::Greater
      } else {
        Ordering::Less
      }
    } else{
      v
    }
  });
  for _ in 0..z {
    result.push(memo.pop().unwrap().2);
  }

  result.sort();
  for v in result {
    println!("{}", v+1);
  }
}
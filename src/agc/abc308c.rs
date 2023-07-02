/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn gcd(a: usize, b: usize) -> usize {
  if b == 0 {
    return a
  }
  gcd(b, a % b)
}

fn main() {
  input! {
    n:usize,
    ab:[(usize,usize);n]
  }

  let mut ab = ab.into_iter()
  .enumerate()
  .map(|(i,(a,b))| {
    let abv = a+b;
    let v = gcd(abv, a);

    (i, (a/v, abv/v))
  }).collect::<Vec<(usize,(usize,usize))>>();

  ab.sort_by(|a,b| {
    if (a.1).0 == (b.1).0 && (a.1).1 == (b.1).1 {
      a.0.cmp(&b.0)
    } else {
      ((b.1).0 * (a.1).1).cmp(&((a.1).0 * (b.1).1))
    }
  });

  println!("{}", ab.into_iter().map(|v| (v.0 + 1).to_string()).collect::<Vec<String>>().join(" "));
}